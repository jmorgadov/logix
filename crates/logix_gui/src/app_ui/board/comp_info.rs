use asmhdl::{AsmComponent, AsmProgram};
use logix_sim::primitives::{data::Data, primitive::Primitive};
use serde::{Deserialize, Serialize};

use super::CompSource;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IOInfo {
    pub name: String,
    pub size: u8,
}

impl IOInfo {
    pub fn new(name: impl Into<String>, bits: u8) -> Self {
        Self {
            name: name.into(),
            size: bits,
        }
    }

    pub fn single(name: impl Into<String>) -> Self {
        Self::new(name, 1)
    }
}

impl std::default::Default for IOInfo {
    fn default() -> Self {
        Self {
            name: String::default(),
            size: 1,
        }
    }
}

impl From<(String, u8)> for IOInfo {
    fn from((name, bits): (String, u8)) -> Self {
        Self { name, size: bits }
    }
}

impl From<&str> for IOInfo {
    fn from(name: &str) -> Self {
        Self::single(name)
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ComponentInfo {
    pub name: String,
    pub description: Option<String>,
    pub source: CompSource,
    pub inputs: Vec<IOInfo>,
    pub outputs: Vec<IOInfo>,
}

impl ComponentInfo {
    pub fn from_asm_comp_code(asm_comp_code: &str) -> Self {
        let asm_comp = AsmComponent::from_code(asm_comp_code);
        Self::custom(
            &asm_comp.name,
            asm_comp
                .inputs
                .iter()
                .map(|(name, size)| IOInfo::new(name, *size))
                .collect(),
            asm_comp
                .outputs
                .iter()
                .map(|(name, size)| IOInfo::new(name, *size))
                .collect(),
            asm_comp.program(),
        )
    }

    pub fn custom(name: &str, inputs: Vec<IOInfo>, outputs: Vec<IOInfo>, prog: AsmProgram) -> Self {
        Self {
            name: name.to_string(),
            source: CompSource::Prim(Primitive::Custom { prog }),
            inputs,
            outputs,
            description: None,
        }
    }

    pub fn and_gate(in_count: u8) -> Self {
        Self {
            name: "AND".to_string(),
            source: CompSource::Prim(Primitive::AndGate),
            inputs: (0..in_count).map(|_| IOInfo::default()).collect(),
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn nand_gate(in_count: u8) -> Self {
        Self {
            name: "NAND".to_string(),
            source: CompSource::Prim(Primitive::NandGate),
            inputs: (0..in_count).map(|_| IOInfo::default()).collect(),
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn or_gate(in_count: u8) -> Self {
        Self {
            name: "OR".to_string(),
            source: CompSource::Prim(Primitive::OrGate),
            inputs: (0..in_count).map(|_| IOInfo::default()).collect(),
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn nor_gate(in_count: u8) -> Self {
        Self {
            name: "NOR".to_string(),
            source: CompSource::Prim(Primitive::NorGate),
            inputs: (0..in_count).map(|_| IOInfo::default()).collect(),
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn xor_gate(in_count: u8) -> Self {
        Self {
            name: "XOR".to_string(),
            source: CompSource::Prim(Primitive::XorGate),
            inputs: (0..in_count).map(|_| IOInfo::default()).collect(),
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn not_gate() -> Self {
        Self {
            name: "NOT".to_string(),
            source: CompSource::Prim(Primitive::NotGate),
            inputs: vec![IOInfo::default()],
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn const_high_gate() -> Self {
        Self {
            name: "HIGH".to_string(),
            source: CompSource::Prim(Primitive::Const {
                value: Data::high(),
            }),
            inputs: vec![],
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn const_low_gate() -> Self {
        Self {
            name: "LOW".to_string(),
            source: CompSource::Prim(Primitive::Const { value: Data::low() }),
            inputs: vec![],
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn clock_gate() -> Self {
        Self {
            name: "CLK".to_string(),
            source: CompSource::Prim(Primitive::Clock {
                period: 1_000_000_000,
            }),
            inputs: vec![],
            outputs: vec![IOInfo::default()],
            description: None,
        }
    }

    pub fn splitter(bits: u8) -> Self {
        Self {
            name: "SPLIT".to_string(),
            source: CompSource::Prim(Primitive::Splitter { bits }),
            inputs: vec![IOInfo::new("", bits)],
            outputs: (0..bits).map(|b| IOInfo::single(b.to_string())).collect(),
            description: None,
        }
    }

    pub fn joiner(bits: u8) -> Self {
        Self {
            name: "JOIN".to_string(),
            source: CompSource::Prim(Primitive::Joiner { bits }),
            inputs: (0..bits).map(|b| IOInfo::single(b.to_string())).collect(),
            outputs: vec![IOInfo::new("", bits)],
            description: None,
        }
    }

    pub fn input(bits: u8) -> Self {
        Self {
            name: "IN".to_string(),
            source: CompSource::Prim(Primitive::Input { bits }),
            inputs: vec![],
            outputs: vec![IOInfo::new("", bits)],
            description: None,
        }
    }

    pub fn output(bits: u8) -> Self {
        Self {
            name: "OUT".to_string(),
            source: CompSource::Prim(Primitive::Output { bits }),
            inputs: vec![IOInfo::new("", bits)],
            outputs: vec![],
            description: None,
        }
    }
}
