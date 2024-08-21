use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlattenComponentError {
    #[error("Failed to reindex connections")]
    BuildError,
}

#[derive(Debug, Error)]
pub enum DataRequestError {
    #[error("Invalid component id: {0}")]
    InvalidComponentId(usize),
    #[error("Invalid input port index: {0}")]
    InvalidInputPortIndex(usize),
    #[error("Invalid output port index: {0}")]
    InvalidOutputPortIndex(usize),
}
