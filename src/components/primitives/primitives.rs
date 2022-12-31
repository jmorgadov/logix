use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Primitive {
    NotGate,
    AndGate,
    OrGate,
    NandGate,
    NorGate,
    XorGate,
    Clock,
    InputPin,
    OutputPin,
    ConstOne,
    ConstZero,
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Primitive {
    pub fn from(name: &str) -> Primitive {
        match name {
            "NotGate" => Self::NotGate,
            "AndGate" => Self::AndGate,
            "OrGate" => Self::OrGate,
            "NandGate" => Self::NandGate,
            "NorGate" => Self::NorGate,
            "XorGate" => Self::XorGate,
            "Clock" => Self::Clock,
            "InputPin" => Self::InputPin,
            "OutputPin" => Self::OutputPin,
            "ConstOne" => Self::ConstOne,
            "ConstZero" => Self::ConstZero,
            _ => panic!("Unkown primitive name '{}'", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Primitive;

    #[test]
    fn primitive_name() {
        assert!(Primitive::AndGate.to_string() == "AndGate");
        assert!(Primitive::Clock.to_string() == "Clock");
    }
}
