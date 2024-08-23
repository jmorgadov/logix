use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub zoom: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { zoom: 1.0 }
    }
}
