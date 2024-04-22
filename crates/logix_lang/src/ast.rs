use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum Primitive {
    And(usize),
    Or(usize),
    Not,
    Xor(usize),
    Nand(usize),
    Nor(usize),
    Clock(f64),
    HighConst,
    LowConst,
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Comp {
    Primitive(Primitive),
    Composite(String),
}

impl Comp {
    pub fn from_name(name: &str, ins_count: usize, clock_frec: f64) -> Self {
        match name {
            "And" => Comp::Primitive(Primitive::And(ins_count)),
            "Or" => Comp::Primitive(Primitive::Or(ins_count)),
            "Not" => Comp::Primitive(Primitive::Not),
            "Xor" => Comp::Primitive(Primitive::Xor(ins_count)),
            "Nand" => Comp::Primitive(Primitive::Nand(ins_count)),
            "Nor" => Comp::Primitive(Primitive::Nor(ins_count)),
            "Clock" => Comp::Primitive(Primitive::Clock(clock_frec)),
            "High" => Comp::Primitive(Primitive::HighConst),
            "Low" => Comp::Primitive(Primitive::LowConst),
            _ => Comp::Composite(name.to_string()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct PinDecl {
    pub name: String,
    pub len: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum PinIndexing {
    NoIndex,
    Index(u8),
    Range(u8, u8),
}

#[derive(Debug, Clone)]
pub enum PinAddr {
    External(String, PinIndexing),
    InternalName(String, String, PinIndexing),
    InternalIdx(String, usize, PinIndexing),
}

#[derive(Debug)]
pub struct ConnDecl {
    pub src: PinAddr,
    pub dest: PinAddr,
}

#[derive(Debug)]
pub struct CompDecl {
    pub name: String,
    pub subc: HashMap<String, Comp>,
    pub ins: HashMap<String, (usize, u8)>,
    pub outs: HashMap<String, (usize, u8)>,
    pub design: Vec<ConnDecl>,
}

#[derive(Debug)]
pub struct Circuit {
    pub comps: Vec<CompDecl>,
}

pub mod prelude {
    pub use super::Circuit;
    pub use super::Comp;
    pub use super::CompDecl;
    pub use super::ConnDecl;
    pub use super::PinAddr;
    pub use super::PinDecl;
    pub use super::Primitive;
    pub use super::PinIndexing;
}
