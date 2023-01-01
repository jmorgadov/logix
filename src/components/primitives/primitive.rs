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

pub struct PrimitiveNotFound;

impl Primitive {
    pub fn from_str(name: &str) -> Result<Primitive, PrimitiveNotFound> {
        match name {
            "NotGate" => Ok(Self::NotGate),
            "AndGate" => Ok(Self::AndGate),
            "OrGate" => Ok(Self::OrGate),
            "NandGate" => Ok(Self::NandGate),
            "NorGate" => Ok(Self::NorGate),
            "XorGate" => Ok(Self::XorGate),
            "Clock" => Ok(Self::Clock),
            "InputPin" => Ok(Self::InputPin),
            "OutputPin" => Ok(Self::OutputPin),
            "ConstOne" => Ok(Self::ConstOne),
            "ConstZero" => Ok(Self::ConstZero),
            _ => Err(PrimitiveNotFound),
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
