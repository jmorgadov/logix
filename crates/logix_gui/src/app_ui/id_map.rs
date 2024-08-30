use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdMap {
    pub id: usize,
    pub name: String,
    pub source: Option<PathBuf>,
    pub sub_ids: Vec<IdMap>,
}

impl IdMap {
    pub const fn new(id: usize, name: String, source: Option<PathBuf>) -> Self {
        Self {
            id,
            name,
            source,
            sub_ids: vec![],
        }
    }

    pub fn ids(&self) -> Vec<usize> {
        self.sub_ids.iter().map(|map| map.id).collect()
    }

    pub fn from_children(
        id: usize,
        name: String,
        source: Option<PathBuf>,
        children: Vec<Self>,
    ) -> Self {
        Self {
            id,
            name,
            source,
            sub_ids: children,
        }
    }

    pub fn id_walk(&self, id_path: &[usize]) -> Option<&Self> {
        let mut current = self;
        for id in id_path {
            current = current.sub_ids.iter().find(|map| map.id == *id)?;
        }
        Some(current)
    }
}
