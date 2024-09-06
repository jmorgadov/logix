use std::path::PathBuf;

use logix_sim::primitives::prelude::Primitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompSource {
    Local(PathBuf),
    Library(String),
    Prim(Primitive),
}

impl std::default::Default for CompSource {
    fn default() -> Self {
        Self::Local("".into())
    }
}

#[allow(dead_code)]
impl CompSource {
    pub fn local_mut(&mut self) -> Option<&mut PathBuf> {
        match self {
            Self::Local(path) => Some(path),
            _ => None,
        }
    }

    pub fn library_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::Library(lib) => Some(lib),
            _ => None,
        }
    }

    pub fn primitive_mut(&mut self) -> Option<&mut Primitive> {
        match self {
            Self::Prim(prim) => Some(prim),
            _ => None,
        }
    }
    pub const fn local(&self) -> Option<&PathBuf> {
        match self {
            Self::Local(path) => Some(path),
            _ => None,
        }
    }

    pub const fn library(&self) -> Option<&String> {
        match self {
            Self::Library(lib) => Some(lib),
            _ => None,
        }
    }

    pub const fn primitive(&self) -> Option<&Primitive> {
        match self {
            Self::Prim(prim) => Some(prim),
            _ => None,
        }
    }
}
