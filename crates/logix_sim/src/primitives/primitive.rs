use std::fmt::{Display, Formatter};

use asmhdl::{AsmComponent, AsmProgramState};
use serde::{Deserialize, Serialize};

use super::data::Data;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Primitive {
    AndGate,
    OrGate,
    NotGate,
    NandGate,
    NorGate,
    XorGate,
    Switch,
    Input {
        bits: u8,
    },
    Output {
        bits: u8,
    },
    Splitter {
        bits: u8,
    },
    Joiner {
        bits: u8,
    },
    Clock {
        period: u128,
    },
    Const {
        value: Data,
    },
    Custom {
        comp: AsmComponent,
        state: AsmProgramState,
    },
}

impl Primitive {
    pub fn input_default_data(self) -> Option<Data> {
        match self {
            Primitive::AndGate
            | Primitive::OrGate
            | Primitive::NotGate
            | Primitive::NandGate
            | Primitive::NorGate
            | Primitive::Joiner { bits: _ }
            | Primitive::Custom { .. }
            | Primitive::Switch
            | Primitive::XorGate => Some(Data::low()),
            Primitive::Output { bits: b } => Some(Data::new(0, b)),
            Primitive::Splitter { bits: b } => Some(Data::new(0, b)),
            Primitive::Input { bits: _ } => None,
            Primitive::Clock { period: _ } => None,
            Primitive::Const { value: _ } => None,
        }
    }

    pub fn output_default_data(self) -> Option<Data> {
        match self {
            Primitive::AndGate
            | Primitive::OrGate
            | Primitive::NotGate
            | Primitive::NandGate
            | Primitive::NorGate
            | Primitive::Switch
            | Primitive::Clock { period: _ }
            | Primitive::Splitter { bits: _ }
            | Primitive::Custom { .. }
            | Primitive::XorGate => Some(Data::low()),
            Primitive::Input { bits: b } => Some(Data::new(0, b)),
            Primitive::Joiner { bits: b } => Some(Data::new(0, b)),
            Primitive::Const { value: v } => Some(v),
            Primitive::Output { bits: _ } => None,
        }
    }

    pub fn is_input(&self) -> bool {
        matches!(self, Primitive::Input { bits: _ })
    }

    pub fn is_output(&self) -> bool {
        matches!(self, Primitive::Output { bits: _ })
    }
}

#[derive(Debug)]
pub struct PrimitiveComponent {
    pub id: usize,
    pub name: String,
    pub prim_type: Primitive,
    pub inputs: Vec<Data>,
    pub outputs: Vec<Data>,
}

impl PrimitiveComponent {
    pub fn set_input(&mut self, index: usize, value: Data) {
        self.inputs[index] = value;
    }

    pub fn update(&mut self, time: u128) {
        match &mut self.prim_type {
            Primitive::AndGate => {
                self.outputs[0].set_bit(self.inputs.iter().fold(true, |acc, &x| acc & x.as_bool()));
            }
            Primitive::OrGate => {
                self.outputs[0]
                    .set_bit(self.inputs.iter().fold(false, |acc, &x| acc | x.as_bool()));
            }
            Primitive::NotGate => {
                self.outputs[0] = !self.inputs[0];
            }
            Primitive::NandGate => {
                self.outputs[0]
                    .set_bit(!self.inputs.iter().fold(true, |acc, &x| acc & x.as_bool()));
            }
            Primitive::NorGate => {
                self.outputs[0]
                    .set_bit(!self.inputs.iter().fold(false, |acc, &x| acc | x.as_bool()));
            }
            Primitive::XorGate => {
                self.outputs[0].set_bit(
                    self.inputs
                        .iter()
                        .skip(1)
                        .fold(self.inputs[0].as_bool(), |acc, &x| acc ^ x.as_bool()),
                );
            }
            Primitive::Input { bits: _b } => {
                self.outputs[0].set_from(self.inputs[0]);
            }
            Primitive::Output { bits: _b } => {
                self.outputs[0].set_from(self.inputs[0]);
            }
            Primitive::Splitter { bits: b } => {
                for bit_idx in (0..*b).rev() {
                    self.outputs[bit_idx as usize].set_bit(self.inputs[0].get_bit_at(bit_idx));
                }
            }
            Primitive::Joiner { bits: _b } => {
                self.outputs[0].set_data(
                    self.inputs
                        .iter()
                        .enumerate()
                        .fold(0, |acc, (i, x)| acc | ((x.as_bool() as usize) << i)),
                );
            }
            Primitive::Clock { period } => {
                self.outputs[0].set_bit((time % (*period * 2)) > *period);
            }
            Primitive::Const { value: _v } => (),
            Primitive::Custom { comp, state } => {
                // Set inputs and outputs in program state for internal access
                comp.inputs.iter().enumerate().for_each(|(idx, (name, _))| {
                    state
                        .vars
                        .insert(name.clone(), self.inputs[idx].as_asm_val());
                });
                comp.outputs
                    .iter()
                    .enumerate()
                    .for_each(|(idx, (name, _))| {
                        state
                            .vars
                            .insert(name.clone(), self.outputs[idx].as_asm_val());
                    });

                state.run(time);

                // Update outputs from program state
                comp.outputs
                    .iter_mut()
                    .enumerate()
                    .for_each(|(idx, (name, _))| {
                        self.outputs[idx] = state.vars[name].into();
                    });
            }
            Primitive::Switch { .. } => {} // Switch only changes on user input
        }
    }

    pub fn custom(id: usize, comp: AsmComponent) -> Self {
        let in_count = comp.inputs.len();
        let out_count = comp.outputs.len();
        let state = comp.new_program_state();
        PrimitiveComponent {
            id,
            name: "Custom".to_string(),
            prim_type: Primitive::Custom { comp, state },
            inputs: vec![Data::low(); in_count],
            outputs: vec![Data::low(); out_count],
        }
    }

    pub fn and_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "And".to_string(),
            prim_type: Primitive::AndGate,
            inputs: vec![Data::low(); in_count],
            outputs: vec![Data::low()],
        }
    }

    pub fn or_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Or".to_string(),
            prim_type: Primitive::OrGate,
            inputs: vec![Data::low(); in_count],
            outputs: vec![Data::low()],
        }
    }

    pub fn not_gate(id: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Not".to_string(),
            prim_type: Primitive::NotGate,
            inputs: vec![Data::low()],
            outputs: vec![Data::low()],
        }
    }

    pub fn nand_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Nand".to_string(),
            prim_type: Primitive::NandGate,
            inputs: vec![Data::low(); in_count],
            outputs: vec![Data::low()],
        }
    }

    pub fn nor_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Nor".to_string(),
            prim_type: Primitive::NorGate,
            inputs: vec![Data::low(); in_count],
            outputs: vec![Data::low()],
        }
    }

    pub fn xor_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Xor".to_string(),
            prim_type: Primitive::XorGate,
            inputs: vec![Data::low(); in_count],
            outputs: vec![Data::low()],
        }
    }

    pub fn switch(id: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Switch".to_string(),
            prim_type: Primitive::Switch,
            inputs: vec![],
            outputs: vec![Data::low()],
        }
    }

    pub fn input(id: usize, bits: u8) -> Self {
        PrimitiveComponent {
            id,
            name: "Input".to_string(),
            prim_type: Primitive::Input { bits },
            inputs: vec![Data::new(0, bits)],
            outputs: vec![Data::new(0, bits)],
        }
    }

    pub fn output(id: usize, bits: u8) -> Self {
        PrimitiveComponent {
            id,
            name: "Output".to_string(),
            prim_type: Primitive::Output { bits },
            inputs: vec![Data::new(0, bits)],
            outputs: vec![Data::new(0, bits)],
        }
    }

    pub fn splitter(id: usize, bits: u8) -> Self {
        PrimitiveComponent {
            id,
            name: "Splitter".to_string(),
            prim_type: Primitive::Splitter { bits },
            inputs: vec![Data::new(0, bits)],
            outputs: vec![Data::low(); bits as usize],
        }
    }

    pub fn joiner(id: usize, bits: u8) -> Self {
        PrimitiveComponent {
            id,
            name: "Joiner".to_string(),
            prim_type: Primitive::Joiner { bits },
            inputs: vec![Data::low(); bits as usize],
            outputs: vec![Data::new(0, bits)],
        }
    }

    pub fn clock(id: usize, period: u128) -> Self {
        PrimitiveComponent {
            id,
            name: format!("Clock({})", 1_000_000_000.0 / period as f64),
            prim_type: Primitive::Clock { period },
            inputs: vec![],
            outputs: vec![Data::low()],
        }
    }

    pub fn const_gate(id: usize, value: Data) -> Self {
        PrimitiveComponent {
            id,
            name: format!("Const({})", value),
            prim_type: Primitive::Const { value },
            inputs: vec![],
            outputs: vec![value],
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExtraInfo {
    pub id: usize,
    pub primitive: Option<Primitive>,
}

impl ExtraInfo {
    pub fn new(id: usize) -> Self {
        ExtraInfo {
            id,
            primitive: None,
        }
    }

    pub fn from_primitive(id: usize, primitive: Primitive) -> Self {
        ExtraInfo {
            id,
            primitive: Some(primitive),
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_truth_table(mut comp: PrimitiveComponent, truth_table: Vec<(Vec<Data>, Vec<Data>)>) {
        for (inputs, outputs) in truth_table {
            for (i, input) in inputs.iter().enumerate() {
                comp.set_input(i, *input);
            }
            comp.update(0);
            for (i, output) in outputs.iter().enumerate() {
                assert_eq!(
                    *output, comp.outputs[i],
                    "Case: {:?} -> {:?}",
                    inputs, output
                );
            }
        }
    }

    #[test]
    fn test_and_gate() {
        let comp = PrimitiveComponent::and_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], vec![Data::high()]),
            (vec![Data::high(), Data::low()], vec![Data::low()]),
            (vec![Data::low(), Data::high()], vec![Data::low()]),
            (vec![Data::low(), Data::low()], vec![Data::low()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_or_gate() {
        let comp = PrimitiveComponent::or_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], vec![Data::high()]),
            (vec![Data::high(), Data::low()], vec![Data::high()]),
            (vec![Data::low(), Data::high()], vec![Data::high()]),
            (vec![Data::low(), Data::low()], vec![Data::low()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_not_gate() {
        let comp = PrimitiveComponent::not_gate(0);
        let truth_table = vec![
            (vec![Data::high()], vec![Data::low()]),
            (vec![Data::low()], vec![Data::high()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_nand_gate() {
        let comp = PrimitiveComponent::nand_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], vec![Data::low()]),
            (vec![Data::high(), Data::low()], vec![Data::high()]),
            (vec![Data::low(), Data::high()], vec![Data::high()]),
            (vec![Data::low(), Data::low()], vec![Data::high()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_nor_gate() {
        let comp = PrimitiveComponent::nor_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], vec![Data::low()]),
            (vec![Data::high(), Data::low()], vec![Data::low()]),
            (vec![Data::low(), Data::high()], vec![Data::low()]),
            (vec![Data::low(), Data::low()], vec![Data::high()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_xor_gate() {
        let comp = PrimitiveComponent::xor_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], vec![Data::low()]),
            (vec![Data::high(), Data::low()], vec![Data::high()]),
            (vec![Data::low(), Data::high()], vec![Data::high()]),
            (vec![Data::low(), Data::low()], vec![Data::low()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_input() {
        let comp = PrimitiveComponent::input(0, 1);
        let truth_table = vec![
            (vec![Data::high()], vec![Data::high()]),
            (vec![Data::low()], vec![Data::low()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_output() {
        let comp = PrimitiveComponent::output(0, 1);
        let truth_table = vec![
            (vec![Data::high()], vec![Data::high()]),
            (vec![Data::low()], vec![Data::low()]),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_splitter() {
        let comp = PrimitiveComponent::splitter(0, 4);
        let truth_table = vec![
            (
                vec![Data::new(0b1110, 4)],
                vec![
                    Data::new(0b0, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                ],
            ),
            (
                vec![Data::new(0b1111, 4)],
                vec![
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                ],
            ),
            (
                vec![Data::new(0b0000, 4)],
                vec![
                    Data::new(0b0, 1),
                    Data::new(0b0, 1),
                    Data::new(0b0, 1),
                    Data::new(0b0, 1),
                ],
            ),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_joiner() {
        let comp = PrimitiveComponent::joiner(0, 4);
        let truth_table = vec![
            (
                vec![
                    Data::new(0b0, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                ],
                vec![Data::new(14, 4)],
            ),
            (
                vec![
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                    Data::new(0b1, 1),
                ],
                vec![Data::new(15, 4)],
            ),
            (
                vec![
                    Data::new(0b0, 1),
                    Data::new(0b0, 1),
                    Data::new(0b0, 1),
                    Data::new(0b0, 1),
                ],
                vec![Data::new(0, 4)],
            ),
            (
                vec![
                    Data::new(0b1, 1),
                    Data::new(0b0, 1),
                    Data::new(0b1, 1),
                    Data::new(0b0, 1),
                ],
                vec![Data::new(5, 4)],
            ),
        ];
        test_truth_table(comp, truth_table);
    }
}
