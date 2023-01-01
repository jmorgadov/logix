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

// This array serves as an iterator over the implemented
// primitives.
pub const PRIMITIVES: [Primitive; 11] = [
    Primitive::NotGate,
    Primitive::AndGate,
    Primitive::OrGate,
    Primitive::NandGate,
    Primitive::NorGate,
    Primitive::XorGate,
    Primitive::Clock,
    Primitive::InputPin,
    Primitive::OutputPin,
    Primitive::ConstOne,
    Primitive::ConstZero,
];

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub struct PrimitiveNotFound;

impl Primitive {
    pub fn from_str(name: &str) -> Result<Primitive, PrimitiveNotFound> {
        for prim in PRIMITIVES {
            if name == prim.to_string() {
                return Ok(prim);
            }
        }
        Err(PrimitiveNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::Primitive;

    #[test]
    fn primitive_name() {
        assert!(Primitive::AndGate.to_string() == "AndGate");
    }
}
