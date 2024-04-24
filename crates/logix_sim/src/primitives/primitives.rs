use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Primitive {
    AndGate,
    OrGate,
    NotGate,
    NandGate,
    NorGate,
    XorGate,
    Clock { period: u128 },
    Const { value: bool },
}

#[derive(Debug)]
pub struct PrimitiveComponent {
    pub name: String,
    pub prim_type: Primitive,
    pub inputs: Vec<bool>,
    pub outputs: Vec<bool>,
}

impl PrimitiveComponent {
    pub fn set_input(&mut self, index: usize, value: bool) {
        self.inputs[index] = value;
    }

    pub fn update(&mut self, time: u128) {
        match self.prim_type {
            Primitive::AndGate => {
                self.outputs[0] = self.inputs.iter().all(|&x| x);
            }
            Primitive::OrGate => {
                self.outputs[0] = self.inputs.iter().any(|&x| x);
            }
            Primitive::NotGate => {
                self.outputs[0] = !self.inputs[0];
            }
            Primitive::NandGate => {
                self.outputs[0] = !self.inputs.iter().all(|&x| x);
            }
            Primitive::NorGate => {
                self.outputs[0] = !self.inputs.iter().any(|&x| x);
            }
            Primitive::XorGate => {
                self.outputs[0] = self.inputs.iter().fold(false, |acc, &x| acc ^ x);
            }
            Primitive::Clock { period } => {
                self.outputs[0] = (time % (period * 2)) > period;
            }
            Primitive::Const { value: _v } => (),
        }
    }

    pub fn and_gate(in_count: usize) -> Self {
        PrimitiveComponent {
            name: "And".to_string(),
            prim_type: Primitive::AndGate,
            inputs: vec![false; in_count],
            outputs: vec![false],
        }
    }

    pub fn or_gate(in_count: usize) -> Self {
        PrimitiveComponent {
            name: "Or".to_string(),
            prim_type: Primitive::OrGate,
            inputs: vec![false; in_count],
            outputs: vec![false],
        }
    }

    pub fn not_gate() -> Self {
        PrimitiveComponent {
            name: "Not".to_string(),
            prim_type: Primitive::NotGate,
            inputs: vec![false],
            outputs: vec![false],
        }
    }

    pub fn nand_gate(in_count: usize) -> Self {
        PrimitiveComponent {
            name: "Nand".to_string(),
            prim_type: Primitive::NandGate,
            inputs: vec![false; in_count],
            outputs: vec![false],
        }
    }

    pub fn nor_gate(in_count: usize) -> Self {
        PrimitiveComponent {
            name: "Nor".to_string(),
            prim_type: Primitive::NorGate,
            inputs: vec![false; in_count],
            outputs: vec![false],
        }
    }

    pub fn xor_gate(in_count: usize) -> Self {
        PrimitiveComponent {
            name: "Xor".to_string(),
            prim_type: Primitive::XorGate,
            inputs: vec![false; in_count],
            outputs: vec![false],
        }
    }

    pub fn clock(period: u128) -> Self {
        PrimitiveComponent {
            name: format!("Clock({})", 1_000_000_000.0 / period as f64),
            prim_type: Primitive::Clock { period },
            inputs: vec![],
            outputs: vec![false],
        }
    }

    pub fn const_gate(value: bool) -> Self {
        PrimitiveComponent {
            name: format!("Const({})", if value { 1 } else { 0 }),
            prim_type: Primitive::Const { value: value },
            inputs: vec![],
            outputs: vec![value],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExtraInfo {
    pub id: String,
    pub primitive: Option<Primitive>,
}

impl ExtraInfo {
    pub fn new(id: String) -> Self {
        ExtraInfo { id, primitive: None }
    }

    pub fn from_primitive(id: String, primitive: Primitive) -> Self {
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
