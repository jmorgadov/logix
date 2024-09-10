//! AsmHDL is a simple hardware description language that encodes hardware components using a
//! simple assembly-like language.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces
)]

mod component;
mod parser;
mod program;
mod value;

pub use component::AsmComponent;
pub use program::{AsmCommand, AsmExpr, AsmProgramState, AsmProgramUpdateType};
pub use value::AsmValue;
