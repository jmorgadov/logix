use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IOInfo {
    pub idx: usize,
    pub name: String,
}

impl IOInfo {
    pub const fn new(idx: usize, name: String) -> Self {
        Self { idx, name }
    }
}
