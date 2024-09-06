use logix_sim::primitives::{data::Data, primitive::Primitive};
use serde::{Deserialize, Serialize};

use super::CompSource;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ComponentInfo {
    pub id: usize,
    pub name: String,
    pub source: CompSource,
    pub inputs_name: Vec<String>,
    pub outputs_name: Vec<String>,
}

impl ComponentInfo {
    pub fn input_count(&self) -> usize {
        self.inputs_name.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs_name.len()
    }

    pub fn and_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "AND".to_string(),
            source: CompSource::Prim(Primitive::AndGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
        }
    }

    pub fn nand_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "NAND".to_string(),
            source: CompSource::Prim(Primitive::NandGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
        }
    }

    pub fn or_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "OR".to_string(),
            source: CompSource::Prim(Primitive::OrGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
        }
    }

    pub fn nor_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "NOR".to_string(),
            source: CompSource::Prim(Primitive::NorGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
        }
    }

    pub fn xor_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "XOR".to_string(),
            source: CompSource::Prim(Primitive::XorGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
        }
    }

    pub fn not_gate(id: usize) -> Self {
        Self {
            id,
            name: "NOT".to_string(),
            source: CompSource::Prim(Primitive::NotGate),
            inputs_name: vec![String::default()],
            outputs_name: vec![String::default()],
        }
    }

    pub fn const_high_gate(id: usize) -> Self {
        Self {
            id,
            name: "HIGH".to_string(),
            source: CompSource::Prim(Primitive::Const {
                value: Data::high(),
            }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
        }
    }

    pub fn const_low_gate(id: usize) -> Self {
        Self {
            id,
            name: "LOW".to_string(),
            source: CompSource::Prim(Primitive::Const { value: Data::low() }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
        }
    }

    pub fn clock_gate(id: usize) -> Self {
        Self {
            id,
            name: "CLK".to_string(),
            source: CompSource::Prim(Primitive::Clock {
                period: 1_000_000_000,
            }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
        }
    }

    pub fn splitter(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "SPLIT".to_string(),
            source: CompSource::Prim(Primitive::Splitter { bits }),
            inputs_name: vec![String::default()],
            outputs_name: (0..bits).map(|b| b.to_string()).collect(),
        }
    }

    pub fn joiner(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "JOIN".to_string(),
            source: CompSource::Prim(Primitive::Joiner { bits }),
            inputs_name: (0..bits).map(|b| b.to_string()).collect(),
            outputs_name: vec![String::default()],
        }
    }

    pub fn input(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "IN".to_string(),
            source: CompSource::Prim(Primitive::Input { bits }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
        }
    }

    pub fn output(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "OUT".to_string(),
            source: CompSource::Prim(Primitive::Output { bits }),
            inputs_name: vec![String::default()],
            outputs_name: vec![],
        }
    }
}
