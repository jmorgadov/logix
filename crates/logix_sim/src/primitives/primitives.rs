use std::fmt::{Display, Formatter};

use super::bit::Data;

#[derive(Debug, Clone)]
pub enum Primitive {
    AndGate,
    OrGate,
    NotGate,
    NandGate,
    NorGate,
    XorGate,
    Input { bits: u8 },
    Output { bits: u8 },
    Clock { period: u128 },
    Const { value: Data },
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
        match self.prim_type {
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
            Primitive::Clock { period } => {
                self.outputs[0].set_bit((time % (period * 2)) > period);
            }
            Primitive::Const { value: _v } => (),
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

    pub fn input(id: usize, bits: u8) -> Self {
        PrimitiveComponent {
            id,
            name: "Input".to_string(),
            prim_type: Primitive::Input { bits },
            inputs: vec![Data::new(0, bits as u8)],
            outputs: vec![Data::new(0, bits as u8)],
        }
    }

    pub fn output(id: usize, bits: u8) -> Self {
        PrimitiveComponent {
            id,
            name: "Output".to_string(),
            prim_type: Primitive::Output { bits },
            inputs: vec![Data::new(0, bits as u8)],
            outputs: vec![Data::new(0, bits as u8)],
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

#[derive(Debug, Default, Clone)]
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

    fn test_truth_table(mut comp: PrimitiveComponent, truth_table: Vec<(Vec<Data>, Data)>) {
        for (inputs, output) in truth_table {
            for (i, input) in inputs.iter().enumerate() {
                comp.set_input(i, *input);
            }
            comp.update(0);
            assert_eq!(
                comp.outputs[0], output,
                "Case: {:?} -> {:?}",
                inputs, output
            );
        }
    }

    #[test]
    fn test_and_gate() {
        let comp = PrimitiveComponent::and_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], Data::high()),
            (vec![Data::high(), Data::low()], Data::low()),
            (vec![Data::low(), Data::high()], Data::low()),
            (vec![Data::low(), Data::low()], Data::low()),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_or_gate() {
        let comp = PrimitiveComponent::or_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], Data::high()),
            (vec![Data::high(), Data::low()], Data::high()),
            (vec![Data::low(), Data::high()], Data::high()),
            (vec![Data::low(), Data::low()], Data::low()),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_not_gate() {
        let comp = PrimitiveComponent::not_gate(0);
        let truth_table = vec![
            (vec![Data::high()], Data::low()),
            (vec![Data::low()], Data::high()),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_nand_gate() {
        let comp = PrimitiveComponent::nand_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], Data::low()),
            (vec![Data::high(), Data::low()], Data::high()),
            (vec![Data::low(), Data::high()], Data::high()),
            (vec![Data::low(), Data::low()], Data::high()),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_nor_gate() {
        let comp = PrimitiveComponent::nor_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], Data::low()),
            (vec![Data::high(), Data::low()], Data::low()),
            (vec![Data::low(), Data::high()], Data::low()),
            (vec![Data::low(), Data::low()], Data::high()),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_xor_gate() {
        let comp = PrimitiveComponent::xor_gate(0, 2);
        let truth_table = vec![
            (vec![Data::high(), Data::high()], Data::low()),
            (vec![Data::high(), Data::low()], Data::high()),
            (vec![Data::low(), Data::high()], Data::high()),
            (vec![Data::low(), Data::low()], Data::low()),
        ];
        test_truth_table(comp, truth_table);
    }
}
