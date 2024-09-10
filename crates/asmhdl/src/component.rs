use crate::{
    program::{AsmCommand, AsmProgramState, AsmProgramUpdateType},
    AsmValue,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component definition
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AsmComponent {
    /// Component name
    pub name: String,

    /// Smalle description of the component
    pub description: Option<String>,

    /// Update type of the component
    pub update_type: AsmProgramUpdateType,

    /// Input ports
    ///
    /// Key: Port name
    /// Value: Port size in bits
    pub inputs: IndexMap<String, u8>,

    /// Output ports
    ///
    /// Key: Port name
    /// Value: Port size in bits
    pub outputs: IndexMap<String, u8>,

    /// Variables defined in by default in the component
    ///
    /// Key: Variable name
    /// Value: Variable value
    pub defaults: HashMap<String, AsmValue>,

    /// Commands that define the component behavior
    pub cmds: Vec<AsmCommand>,
}

impl AsmComponent {
    /// Parses a component from a asmhdl file
    pub fn from_file(path: &str) -> Self {
        let text = std::fs::read_to_string(path).expect("Failed to read file");
        Self::parse(&text)
    }

    /// Parses a component from an asmhdl code
    pub fn from_code(code: &str) -> Self {
        Self::parse(code)
    }

    /// Generates an [`AsmProgram`] from the component information
    pub fn new_program_state(&self) -> AsmProgramState {
        AsmProgramState::new(self.cmds.clone()).with_default_vars(self.defaults.clone())
    }

    /// Creates a new empty component with the given name
    ///
    /// Use the builder methods to add information to the component
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Sets the component [`AsmComponent::description`] value
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Sets the component [`AsmComponent::update_type`] value
    pub fn with_update(mut self, update_type: AsmProgramUpdateType) -> Self {
        self.update_type = update_type;
        self
    }

    /// Adds an input port to the component
    ///
    /// The order of the calls to this method will define the order of the input ports
    pub fn with_input(mut self, name: &str, size: u8) -> Self {
        self.inputs.insert(name.to_string(), size);
        self
    }

    /// Adds an output port to the component
    ///
    /// The order of the calls to this method will define the order of the output ports
    pub fn with_output(mut self, name: &str, size: u8) -> Self {
        self.outputs.insert(name.to_string(), size);
        self
    }

    /// Adds a default variable to the component
    pub fn with_default(mut self, name: &str, value: AsmValue) -> Self {
        self.defaults.insert(name.to_string(), value);
        self
    }

    /// Adds commands to the component's behavior
    ///
    /// The order of the calls to this method will define the order of the commands
    pub fn with_cmds(mut self, cmd: Vec<AsmCommand>) -> Self {
        self.cmds.extend(cmd);
        self
    }
}
