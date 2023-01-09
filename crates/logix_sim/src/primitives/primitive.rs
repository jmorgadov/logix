use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum Primitive {
    NotGate,
    AndGate,
    OrGate,
    NandGate,
    NorGate,
    XorGate,
    Clock,
    HighConst,
    LowConst,
    Unknown,
}

pub const PRIMITIVES: [Primitive; 10] = [
    Primitive::NotGate,
    Primitive::AndGate,
    Primitive::OrGate,
    Primitive::NandGate,
    Primitive::NorGate,
    Primitive::XorGate,
    Primitive::Clock,
    Primitive::HighConst,
    Primitive::LowConst,
    Primitive::Unknown,
];

impl Primitive {
    /// Returns the primitive type given the component's name. If the name
    /// does not match with any primitive, the [`Primitive::Unknown`] value
    /// is returned;
    ///
    /// The name of a primitive is the same as the enum value:
    ///
    /// ```
    /// # use logix_sim::primitives::prelude::*;
    /// #
    /// assert!(Primitive::Clock == Primitive::from_name("Clock"))
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
