use serde::{Deserialize, Serialize};

use super::board::CompSource;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdMap {
    pub id: usize,
    pub name: String,
    pub source: CompSource,
    pub sub_ids: Vec<IdMap>,
}

impl IdMap {
    pub const fn new(id: usize, name: String, source: CompSource) -> Self {
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

    pub const fn from_children(
        id: usize,
        name: String,
        source: CompSource,
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
