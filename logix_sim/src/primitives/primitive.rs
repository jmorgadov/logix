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
    Const,
    Unknown,
}

pub const PRIMITIVES: [Primitive; 9] = [
    Primitive::NotGate,
    Primitive::AndGate,
    Primitive::OrGate,
    Primitive::NandGate,
    Primitive::NorGate,
    Primitive::XorGate,
    Primitive::Clock,
    Primitive::Const,
    Primitive::Unknown,
];

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
    pub fn from_name(name: &str) -> Primitive {
        for prim in PRIMITIVES {
            if name == prim.to_string() {
                return prim;
            }
        }
        Primitive::Unknown
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
