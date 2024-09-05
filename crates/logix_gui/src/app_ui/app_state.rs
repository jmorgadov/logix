#[derive(Debug, Default, Clone)]
pub enum LeftPannelState {
    #[default]
    Folders,
    Design,
    Simulation,
}

#[derive(Debug, Default, Clone)]
pub enum AppState {
    #[default]
    OnWelcome,
    CreatingNewProject {
        folder: String,
        name: String,
    },
    OnProject(LeftPannelState),
}

impl AppState {
    pub fn new_project_folder(&mut self) -> &mut String {
        match self {
            Self::CreatingNewProject { folder, name: _ } => folder,
            _ => panic!("Not in new project state"),
        }
    }

    pub fn new_project_name(&mut self) -> &mut String {
        match self {
            Self::CreatingNewProject { folder: _, name } => name,
            _ => panic!("Not in new project state"),
        }
    }
}
