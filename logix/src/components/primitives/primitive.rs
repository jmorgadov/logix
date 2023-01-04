use std::fmt::{Display, Formatter};

/// Enum that contains all primitive component types implemented.
#[derive(Debug, PartialEq, Eq)]
pub enum Primitive {
    NotGate,
    AndGate,
    OrGate,
    NandGate,
    NorGate,
    XorGate,
    Clock,
    ConstOne,
    ConstZero,
}

/// Primitive enum values stored in an array.
// This array serves as an iterator over the implemented
// primitives.
pub const PRIMITIVES: [Primitive; 9] = [
    Primitive::NotGate,
    Primitive::AndGate,
    Primitive::OrGate,
    Primitive::NandGate,
    Primitive::NorGate,
    Primitive::XorGate,
    Primitive::Clock,
    Primitive::ConstOne,
    Primitive::ConstZero,
];

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

/// Error that describes the use of a not implemented primitive (e.g., when
/// getting the primitive enum value given its name).
#[derive(Debug)]
pub struct PrimitiveNotFound;

impl Primitive {
    /// Returns a `Result` that contains the primitive enum value (`Ok(Primitive)`)
    /// given its name as string. If the primitive name is invalid an
    /// `Err(PrimitiveNotFound)` is returned.
    ///
    /// The name of a primitive is the same as the enum value:
    ///
    /// ```
    /// use logix::prelude::*;
    /// assert!(Primitive::Clock == Primitive::from_name("Clock").unwrap())
    /// ```
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the primitive name.
    pub fn from_name(name: &str) -> Result<Primitive, PrimitiveNotFound> {
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
