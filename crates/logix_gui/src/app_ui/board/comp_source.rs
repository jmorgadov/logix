use std::path::PathBuf;

use logix_sim::primitives::prelude::Primitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompSource {
    Local(PathBuf),
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
            Self::Prim(_) => None,
        }
    }

    pub fn primitive_mut(&mut self) -> Option<&mut Primitive> {
        match self {
            Self::Prim(prim) => Some(prim),
            Self::Local(_) => None,
        }
    }

    pub const fn local(&self) -> Option<&PathBuf> {
        match self {
            Self::Local(path) => Some(path),
            Self::Prim(_) => None,
        }
    }

    pub const fn primitive(&self) -> Option<&Primitive> {
        match self {
            Self::Prim(prim) => Some(prim),
            Self::Local(_) => None,
        }
    }
}
