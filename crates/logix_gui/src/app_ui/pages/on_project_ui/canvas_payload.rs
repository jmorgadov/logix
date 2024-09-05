use std::path::PathBuf;

use crate::app_ui::board::BoardComponent;

#[derive(Debug)]
pub enum CanvasPayload {
    Component(BoardComponent),
    Path(PathBuf),
}
