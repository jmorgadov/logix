use egui::Pos2;
use logix_sim::primitives::{data::Data, prelude::Primitive};
use serde::{Deserialize, Serialize};

use super::comp_info::ComponentInfo;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BoardComponent {
    pub pos: Pos2,
    pub info: ComponentInfo,
    pub inputs_data: Vec<Data>,
    pub outputs_data: Vec<Data>,
}

impl BoardComponent {
    pub fn input_count(&self) -> usize {
        self.inputs_data.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs_data.len()
    }

    pub fn is_input(&self) -> bool {
        self.info
            .primitive
            .as_ref()
            .is_some_and(Primitive::is_input)
    }

    pub fn is_output(&self) -> bool {
        self.info
            .primitive
            .as_ref()
            .is_some_and(Primitive::is_output)
    }

    pub const fn with_pos(mut self, pos: Pos2) -> Self {
        self.pos = pos;
        self
    }

    pub fn and_gate(id: usize, in_count: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::and_gate(id, in_count),
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn nand_gate(id: usize, in_count: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::nand_gate(id, in_count),
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn or_gate(id: usize, in_count: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::or_gate(id, in_count),
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn xor_gate(id: usize, in_count: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::xor_gate(id, in_count),
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn nor_gate(id: usize, in_count: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::nor_gate(id, in_count),
            inputs_data: vec![Data::low(); in_count],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn not_gate(id: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::not_gate(id),
            inputs_data: vec![Data::low()],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn const_high_gate(id: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::const_high_gate(id),
            inputs_data: vec![],
            outputs_data: vec![Data::high()],
        }
    }

    pub fn const_low_gate(id: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::const_low_gate(id),
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn clock_gate(id: usize) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::clock_gate(id),
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn splitter(id: usize, bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::splitter(id, bits),
            inputs_data: vec![Data::new(0, bits)],
            outputs_data: vec![Data::low(); bits as usize],
        }
    }

    pub fn joiner(id: usize, bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::joiner(id, bits),
            inputs_data: vec![Data::low(); bits as usize],
            outputs_data: vec![Data::new(0, bits)],
        }
    }

    pub fn input(id: usize, bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::input(id, bits),
            inputs_data: vec![],
            outputs_data: vec![Data::new(0, bits)],
        }
    }

    pub fn output(id: usize, bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::output(id, bits),
            inputs_data: vec![Data::new(0, bits)],
            outputs_data: vec![],
        }
    }
}
