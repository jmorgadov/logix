use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlattenComponentError {
    #[error("Failed to reindex connections")]
    BuildError,
}

#[derive(Debug, Error)]
pub enum ComponentRequestError {
    #[error("Invalid component id: {0}")]
    InvalidComponentId(usize),
    #[error("Component is not primitive")]
    NonPrimitive,
}

#[derive(Debug, Error)]
pub enum DataRequestError {
    #[error("Invalid component id: {0}")]
    InvalidComponentId(#[from] ComponentRequestError),
    #[error("Invalid input port index: {0}")]
    InvalidInputPortIndex(usize),
    #[error("Invalid output port index: {0}")]
    InvalidOutputPortIndex(usize),
}
