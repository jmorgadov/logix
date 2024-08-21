use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadBoardError {
    #[error("Failed to read board file.\n{0}")]
    ReadBoardFile(#[from] std::io::Error),
    #[error("Failed to parse board file.\n{0}")]
    ParseBoardFile(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum SaveBoardError {
    #[error("Failed to write board file.\n{0}")]
    WriteBoardFile(#[from] std::io::Error),
    #[error("Failed to serialize board.\n{0}")]
    SerializeBoard(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum BoardBuildError {
    #[error("Primitive not specified")]
    PrimitiveNotSpecified,
    #[error("Source not specified")]
    SourceNotSpecified,
    #[error("Failed to load board.\n{0}")]
    LoadBoard(#[from] LoadBoardError),
}

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Error building component.\n{0}")]
    BuildComponentError(#[from] BoardBuildError),
    #[error("Error flattening component")]
    FlattenComponentError(#[from] logix_sim::errors::FlattenComponentError),
    #[error("Error getting component data.\nComponent: {comp_name}")]
    RequestComponentData {
        comp_name: String,
        comp_id: usize,
        #[source]
        err: logix_sim::errors::DataRequestError,
    },
}

#[derive(Debug, Error)]
#[error("Failed to load component.\n{0}")]
pub struct LoadComponentError(#[from] LoadBoardError);

#[derive(Debug, Error)]
pub enum ReloadComponentsError {
    #[error("Failed to load component.\n{0}")]
    LoadError(#[from] LoadComponentError),
}

#[derive(Debug, Error)]
pub enum OpenBoardError {
    #[error("Failed to load board.\n{0}")]
    LoadBoard(#[from] LoadBoardError),
    #[error("Failed to reload load component.\n{0}")]
    ReloadComponents(#[from] ReloadComponentsError),
}
