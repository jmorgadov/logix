use std::path::PathBuf;

use crate::app_ui::board::ComponentInfo;

#[derive(Debug)]
pub enum CanvasPayload {
    Component(ComponentInfo),
    Path(PathBuf),
}
