use std::path::PathBuf;

use egui::Pos2;
use logix_sim::primitives::{data::Data, primitive::Primitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConnectionInfo {
    pub points: Vec<Pos2>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentInfo {
    pub id: usize,
    pub name: String,
    pub source: Option<PathBuf>,
    pub primitive: Option<Primitive>,
    pub inputs_name: Vec<String>,
    pub outputs_name: Vec<String>,
    pub inputs_data: Vec<Data>,
    pub outputs_data: Vec<Data>,

    pub inputs_data_idx: Vec<(usize, usize)>,
    pub outputs_data_idx: Vec<(usize, usize)>,
}

impl ComponentInfo {
    pub fn input_count(&self) -> usize {
        self.inputs_data.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs_data.len()
    }

    pub fn and_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "AND".to_string(),
            source: None,
            primitive: Some(Primitive::AndGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![(id, 0); in_count],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn nand_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "NAND".to_string(),
            source: None,
            primitive: Some(Primitive::NandGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![(0, 0); in_count],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn or_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "OR".to_string(),
            source: None,
            primitive: Some(Primitive::OrGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![(0, 0); in_count],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn nor_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "NOR".to_string(),
            source: None,
            primitive: Some(Primitive::NorGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![(0, 0); in_count],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn xor_gate(id: usize, in_count: usize) -> Self {
        Self {
            id,
            name: "XOR".to_string(),
            source: None,
            primitive: Some(Primitive::XorGate),
            inputs_name: (0..in_count).map(|_| String::default()).collect(),
            outputs_name: vec![String::default()],
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![(0, 0); in_count],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn not_gate(id: usize) -> Self {
        Self {
            id,
            name: "NOT".to_string(),
            source: None,
            primitive: Some(Primitive::NotGate),
            inputs_name: vec![String::default()],
            outputs_name: vec![String::default()],
            inputs_data: vec![Data::low()],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![(0, 0)],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn const_high_gate(id: usize) -> Self {
        Self {
            id,
            name: "HIGH".to_string(),
            source: None,
            primitive: Some(Primitive::Const {
                value: Data::high(),
            }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::high()],
            inputs_data_idx: vec![],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn const_low_gate(id: usize) -> Self {
        Self {
            id,
            name: "LOW".to_string(),
            source: None,
            primitive: Some(Primitive::Const { value: Data::low() }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn clock_gate(id: usize) -> Self {
        Self {
            id,
            name: "CLK".to_string(),
            source: None,
            primitive: Some(Primitive::Clock {
                period: 1_000_000_000,
            }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
            inputs_data_idx: vec![],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn splitter(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "SPLIT".to_string(),
            source: None,
            primitive: Some(Primitive::Splitter { bits }),
            inputs_name: vec![String::default()],
            outputs_name: (0..bits).map(|b| b.to_string()).collect(),
            inputs_data: vec![Data::new(0, bits)],
            outputs_data: vec![Data::low(); bits as usize],
            inputs_data_idx: vec![(0, 0)],
            outputs_data_idx: vec![(0, 0); bits as usize],
        }
    }

    pub fn joiner(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "JOIN".to_string(),
            source: None,
            primitive: Some(Primitive::Joiner { bits }),
            inputs_name: (0..bits).map(|b| b.to_string()).collect(),
            outputs_name: vec![String::default()],
            inputs_data: vec![Data::low(); bits as usize],
            outputs_data: vec![Data::new(0, bits)],
            inputs_data_idx: vec![(0, 0); bits as usize],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn input(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "IN".to_string(),
            source: None,
            primitive: Some(Primitive::Input { bits }),
            inputs_name: vec![],
            outputs_name: vec![String::default()],
            inputs_data: vec![],
            outputs_data: vec![Data::new(0, bits)],
            inputs_data_idx: vec![],
            outputs_data_idx: vec![(0, 0)],
        }
    }

    pub fn output(id: usize, bits: u8) -> Self {
        Self {
            id,
            name: "OUT".to_string(),
            source: None,
            primitive: Some(Primitive::Output { bits }),
            inputs_name: vec![String::default()],
            outputs_name: vec![],
            inputs_data: vec![Data::new(0, bits)],
            outputs_data: vec![],
            inputs_data_idx: vec![(0, 0)],
            outputs_data_idx: vec![],
        }
    }
}
