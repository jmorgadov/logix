use std::fmt::{Display, Formatter};

use super::bit::Bit;

#[derive(Debug, Clone)]
pub enum Primitive {
    AndGate,
    OrGate,
    NotGate,
    NandGate,
    NorGate,
    XorGate,
    Clock { period: u128 },
    Const { value: Bit },
}

#[derive(Debug)]
pub struct PrimitiveComponent {
    pub id: usize,
    pub name: String,
    pub prim_type: Primitive,
    pub inputs: Vec<Bit>,
    pub outputs: Vec<Bit>,
}

impl PrimitiveComponent {
    pub fn set_input(&mut self, index: usize, value: Bit) {
        self.inputs[index] = value;
    }

    pub fn update(&mut self, time: u128) {
        match self.prim_type {
            Primitive::AndGate => {
                self.outputs[0] = self.inputs.iter().fold(Bit::High, |acc, &x| acc & x)
            }
            Primitive::OrGate => {
                self.outputs[0] = self.inputs.iter().fold(Bit::Low, |acc, &x| acc | x)
            }
            Primitive::NotGate => {
                self.outputs[0] = !self.inputs[0];
            }
            Primitive::NandGate => {
                self.outputs[0] = !self.inputs.iter().fold(Bit::High, |acc, &x| acc & x)
            }
            Primitive::NorGate => {
                self.outputs[0] = !self.inputs.iter().fold(Bit::Low, |acc, &x| acc | x)
            }
            Primitive::XorGate => {
                self.outputs[0] = self
                    .inputs
                    .iter()
                    .skip(1)
                    .fold(self.inputs[0], |acc, &x| acc ^ x)
            }
            Primitive::Clock { period } => {
                self.outputs[0] = Bit::from_bool((time % (period * 2)) > period);
            }
            Primitive::Const { value: _v } => (),
        }
    }

    pub fn and_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "And".to_string(),
            prim_type: Primitive::AndGate,
            inputs: vec![Bit::Low; in_count],
            outputs: vec![Bit::Low],
        }
    }

    pub fn or_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Or".to_string(),
            prim_type: Primitive::OrGate,
            inputs: vec![Bit::Low; in_count],
            outputs: vec![Bit::Low],
        }
    }

    pub fn not_gate(id: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Not".to_string(),
            prim_type: Primitive::NotGate,
            inputs: vec![Bit::Low],
            outputs: vec![Bit::Low],
        }
    }

    pub fn nand_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Nand".to_string(),
            prim_type: Primitive::NandGate,
            inputs: vec![Bit::Low; in_count],
            outputs: vec![Bit::Low],
        }
    }

    pub fn nor_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Nor".to_string(),
            prim_type: Primitive::NorGate,
            inputs: vec![Bit::Low; in_count],
            outputs: vec![Bit::Low],
        }
    }

    pub fn xor_gate(id: usize, in_count: usize) -> Self {
        PrimitiveComponent {
            id,
            name: "Xor".to_string(),
            prim_type: Primitive::XorGate,
            inputs: vec![Bit::Low; in_count],
            outputs: vec![Bit::Low],
        }
    }

    pub fn clock(id: usize, period: u128) -> Self {
        PrimitiveComponent {
            id,
            name: format!("Clock({})", 1_000_000_000.0 / period as f64),
            prim_type: Primitive::Clock { period },
            inputs: vec![],
            outputs: vec![Bit::Low],
        }
    }

    pub fn const_gate(id: usize, value: Bit) -> Self {
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

    fn test_truth_table(mut comp: PrimitiveComponent, truth_table: Vec<(Vec<Bit>, Bit)>) {
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
            (vec![Bit::High, Bit::High], Bit::High),
            (vec![Bit::High, Bit::Low], Bit::Low),
            (vec![Bit::Low, Bit::High], Bit::Low),
            (vec![Bit::Low, Bit::Low], Bit::Low),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_or_gate() {
        let comp = PrimitiveComponent::or_gate(0, 2);
        let truth_table = vec![
            (vec![Bit::High, Bit::High], Bit::High),
            (vec![Bit::High, Bit::Low], Bit::High),
            (vec![Bit::Low, Bit::High], Bit::High),
            (vec![Bit::Low, Bit::Low], Bit::Low),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_not_gate() {
        let comp = PrimitiveComponent::not_gate(0);
        let truth_table = vec![(vec![Bit::High], Bit::Low), (vec![Bit::Low], Bit::High)];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_nand_gate() {
        let comp = PrimitiveComponent::nand_gate(0, 2);
        let truth_table = vec![
            (vec![Bit::High, Bit::High], Bit::Low),
            (vec![Bit::High, Bit::Low], Bit::High),
            (vec![Bit::Low, Bit::High], Bit::High),
            (vec![Bit::Low, Bit::Low], Bit::High),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_nor_gate() {
        let comp = PrimitiveComponent::nor_gate(0, 2);
        let truth_table = vec![
            (vec![Bit::High, Bit::High], Bit::Low),
            (vec![Bit::High, Bit::Low], Bit::Low),
            (vec![Bit::Low, Bit::High], Bit::Low),
            (vec![Bit::Low, Bit::Low], Bit::High),
        ];
        test_truth_table(comp, truth_table);
    }

    #[test]
    fn test_xor_gate() {
        let comp = PrimitiveComponent::xor_gate(0, 2);
        let truth_table = vec![
            (vec![Bit::High, Bit::High], Bit::Low),
            (vec![Bit::High, Bit::Low], Bit::High),
            (vec![Bit::Low, Bit::High], Bit::High),
            (vec![Bit::Low, Bit::Low], Bit::Low),
        ];
        test_truth_table(comp, truth_table);
    }
}
