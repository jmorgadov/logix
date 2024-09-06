use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Hash)]
pub struct BoardIO {
    pub idx: usize,
    pub name: String,
}

impl BoardIO {
    pub const fn new(idx: usize, name: String) -> Self {
        Self { idx, name }
    }
}
