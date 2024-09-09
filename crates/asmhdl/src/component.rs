use std::collections::HashMap;

use crate::{
    program::{AsmCommand, AsmProgramUpdateType},
    AsmValue,
};

#[derive(Debug, Default)]
pub struct AsmComponent {
    pub info: AsmCompInfo,
    pub inputs: HashMap<String, usize>,
    pub outputs: HashMap<String, usize>,

    pub defaults: HashMap<String, AsmValue>,
    pub cmds: Vec<AsmCommand>,
}

#[derive(Debug, Default)]
pub struct AsmCompInfo {
    pub name: String,
    pub description: Option<String>,
    pub update_type: AsmProgramUpdateType,
}

impl AsmComponent {
    pub fn from_file(path: &str) -> Self {
        let text = std::fs::read_to_string(path).expect("Failed to read file");
        Self::parse(&text)
    }

    pub fn new(name: &str) -> Self {
        Self {
            info: AsmCompInfo {
                name: name.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.info.description = Some(desc.to_string());
        self
    }

    pub fn with_update(mut self, update_type: AsmProgramUpdateType) -> Self {
        self.info.update_type = update_type;
        self
    }

    pub fn with_input(mut self, name: &str, size: usize) -> Self {
        self.inputs.insert(name.to_string(), size);
        self
    }

    pub fn with_output(mut self, name: &str, size: usize) -> Self {
        self.outputs.insert(name.to_string(), size);
        self
    }

    pub fn with_default(mut self, name: &str, value: AsmValue) -> Self {
        self.defaults.insert(name.to_string(), value);
        self
    }

    pub fn with_cmds(mut self, cmd: Vec<AsmCommand>) -> Self {
        self.cmds.extend(cmd);
        self
    }
}
