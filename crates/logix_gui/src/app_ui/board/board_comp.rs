use egui::Pos2;
use logix_sim::{
    pcmd, pexp,
    primitives::{
        data::Data,
        prelude::Primitive,
        prim_program::{PrimProgram, ProgramUpdateType},
    },
};
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
            .source
            .primitive()
            .is_some_and(Primitive::is_input)
    }

    pub fn is_output(&self) -> bool {
        self.info
            .source
            .primitive()
            .is_some_and(Primitive::is_output)
    }

    pub const fn with_pos(mut self, pos: Pos2) -> Self {
        self.pos = pos;
        self
    }

    pub const fn with_id(mut self, id: usize) -> Self {
        self.info.id = id;
        self
    }

    pub fn jkms_flip_flop() -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::custom(
                0,
                "JKMS",
                vec!["J".into(), "CLK".into(), "K".into()],
                vec!["Q".into(), "!Q".into()],
                PrimProgram::new(
                    ProgramUpdateType::InputChanges,
                    vec![
                        // Estimate falling edge
                        pcmd!(
                            mov,
                            "falling_edge",
                            // Falling edge detection (!last_clk && clk)
                            pexp!(and, pexp!(not, pexp!(var, "_i_1")), pexp!(var, "last_clk"))
                        ),
                        // Update last clk with current clk value
                        pcmd!(mov, "last_clk", pexp!(var, "_i_1")),
                        // Check if falling edge
                        pcmd!(cmp_var, "falling_edge", pexp!(data, true)),
                        // If not rising edge, jump to END
                        pcmd!(jne, "END"),
                        // Else ...
                        // Create JK variable
                        pcmd!(
                            mov,
                            "JK",
                            pexp!(bit_vec, pexp!(var, "_i_0"), pexp!(var, "_i_2"))
                        ),
                        // Check JK value and jump to the corresponding case
                        pcmd!(cmp_var, "JK", pexp!(data, "00")),
                        pcmd!(je, "CASE_00"),
                        pcmd!(cmp_var, "JK", pexp!(data, "10")),
                        pcmd!(je, "CASE_10"),
                        pcmd!(cmp_var, "JK", pexp!(data, "01")),
                        pcmd!(je, "CASE_01"),
                        pcmd!(cmp_var, "JK", pexp!(data, "11")),
                        pcmd!(je, "CASE_11"),
                        //
                        pcmd!(label, "CASE_00"),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "CASE_01"),
                        pcmd!(mov, "_o_0", pexp!(data, false)),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "CASE_10"),
                        pcmd!(mov, "_o_0", pexp!(data, true)),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "CASE_11"),
                        pcmd!(mov, "_o_0", pexp!(not, pexp!(var, "_o_0"))),
                        pcmd!(jne, "END"),
                        //
                        pcmd!(label, "END"),
                        // Update !Q
                        pcmd!(mov, "_o_1", pexp!(not, pexp!(var, "_o_0"))),
                    ],
                )
                .with_default_vars([("last_clk", false)].into()),
            ),
            inputs_data: vec![Data::low(); 3],
            outputs_data: vec![Data::low(); 2],
        }
    }

    pub fn and_gate(in_count: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::and_gate(0, in_count),
            inputs_data: vec![Data::low(); in_count as usize],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn nand_gate(in_count: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::nand_gate(0, in_count),
            inputs_data: vec![Data::low(); in_count as usize],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn or_gate(in_count: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::or_gate(0, in_count),
            inputs_data: vec![Data::low(); in_count as usize],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn xor_gate(in_count: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::xor_gate(0, in_count),
            inputs_data: vec![Data::low(); in_count as usize],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn nor_gate(in_count: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::nor_gate(0, in_count),
            inputs_data: vec![Data::low(); in_count as usize],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn not_gate() -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::not_gate(0),
            inputs_data: vec![Data::low()],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn const_high_gate() -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::const_high_gate(0),
            inputs_data: vec![],
            outputs_data: vec![Data::high()],
        }
    }

    pub fn const_low_gate() -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::const_low_gate(0),
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn clock_gate() -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::clock_gate(0),
            inputs_data: vec![],
            outputs_data: vec![Data::low()],
        }
    }

    pub fn splitter(bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::splitter(0, bits),
            inputs_data: vec![Data::new(0, bits)],
            outputs_data: vec![Data::low(); bits as usize],
        }
    }

    pub fn joiner(bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::joiner(0, bits),
            inputs_data: vec![Data::low(); bits as usize],
            outputs_data: vec![Data::new(0, bits)],
        }
    }

    pub fn input(bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::input(0, bits),
            inputs_data: vec![],
            outputs_data: vec![Data::new(0, bits)],
        }
    }

    pub fn output(bits: u8) -> Self {
        Self {
            pos: Pos2::default(),
            info: ComponentInfo::output(0, bits),
            inputs_data: vec![Data::new(0, bits)],
            outputs_data: vec![],
        }
    }
}
