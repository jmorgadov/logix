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
    Input(usize),
    Output(usize),
    Splitter(usize),
    Joiner(usize),
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
            "In" => Comp::Primitive(Primitive::Input(ins_count)),
            "Out" => Comp::Primitive(Primitive::Output(ins_count)),
            "Splitter" => Comp::Primitive(Primitive::Splitter(ins_count)),
            "Joiner" => Comp::Primitive(Primitive::Joiner(ins_count)),
            _ => Comp::Composite(name.to_string()),
        }
    }

    pub fn is_input(&self) -> bool {
        matches!(self, Comp::Primitive(Primitive::Input(_)))
    }

    pub fn is_output(&self) -> bool {
        matches!(self, Comp::Primitive(Primitive::Output(_)))
    }
}

#[derive(Debug, Clone)]
pub enum PinAddr {
    ByName(String, String),
    ByIdx(String, usize),
}

impl PinAddr {
    pub fn name(&self) -> &str {
        match self {
            PinAddr::ByName(n, _) => n,
            PinAddr::ByIdx(n, _) => n,
        }
    }
}

#[derive(Debug)]
pub struct ConnDecl {
    pub src: PinAddr,
    pub dest: PinAddr,
}

#[derive(Debug)]
pub struct CompDecl {
    pub name: String,
    pub subc: HashMap<(String, usize), Comp>,
    pub design: Vec<ConnDecl>,

    pub ins: Vec<usize>,
    pub outs: Vec<usize>,

    pub in_idx_by_name: HashMap<String, usize>,
    pub out_idx_by_name: HashMap<String, usize>,
}

impl CompDecl {
    pub fn new(name: String, subc: HashMap<(String, usize), Comp>, design: Vec<ConnDecl>) -> Self {
        let mut ins = subc
            .iter()
            .filter(|((_, _), c)| c.is_input())
            .collect::<Vec<_>>();

        let mut outs = subc
            .iter()
            .filter(|((_, _), c)| c.is_output())
            .collect::<Vec<_>>();

        ins.sort_by(|((_, a), _), ((_, b), _)| a.cmp(b));
        outs.sort_by(|((_, a), _), ((_, b), _)| a.cmp(b));

        let in_idx_by_name: HashMap<String, usize> = ins
            .iter()
            .enumerate()
            .map(|(i, ((n, _), _))| (n.clone(), i))
            .collect();

        let out_idx_by_name: HashMap<String, usize> = outs
            .iter()
            .enumerate()
            .map(|(i, ((n, _), _))| (n.clone(), i))
            .collect();

        let ins = ins.iter().map(|((_, i), _)| *i).collect();
        let outs = outs.iter().map(|((_, i), _)| *i).collect();

        CompDecl {
            name,
            subc,
            design,
            ins,
            outs,
            in_idx_by_name,
            out_idx_by_name,
        }
    }
}

#[derive(Debug)]
pub struct Circuit {
    pub imports: Option<Vec<String>>,
    pub comps: Vec<CompDecl>,
}

pub mod prelude {
    pub use super::Circuit;
    pub use super::Comp;
    pub use super::CompDecl;
    pub use super::ConnDecl;
    pub use super::PinAddr;
    pub use super::Primitive;
}
