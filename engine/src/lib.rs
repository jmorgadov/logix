pub mod components;
pub mod parser;
pub mod simulation;
pub mod visitor;

pub mod prelude {
    pub use crate::components::prelude::*;
    pub use crate::parser::{CompParser, ParseError, ParseResult};
    pub use crate::simulation::Simulation;
    pub use crate::visitor::CompVisitor;
}
