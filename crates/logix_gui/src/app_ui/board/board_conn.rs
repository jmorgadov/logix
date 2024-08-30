use egui::Pos2;
use logix_core::prelude::Conn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BoardConnection {
    pub conn: Conn,
    pub points: Vec<Pos2>,
}
