use asmhdl::Data;
use egui::Pos2;
use logix_core::prelude::Component;
use logix_sim::primitives::prelude::{ExtraInfo, Primitive};
use serde::{Deserialize, Serialize};

use crate::app_ui::{errors::BoardBuildError, id_map::IdMap};

use super::{comp_info::ComponentInfo, Board, CompSource};

#[derive(Debug, Clone)]
pub enum UserInteraction {
    #[allow(dead_code)]
    ChangeInput(usize, Data),
    ChangeOutput(usize, Data),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BoardComponent {
    pub pos: Pos2,
    pub id: usize,
    pub info: ComponentInfo,
    pub inputs_data: Vec<Data>,
    pub outputs_data: Vec<Data>,

    #[serde(skip)]
    pub user_interaction: Option<Vec<UserInteraction>>,
}

impl BoardComponent {
    pub fn input_count(&self) -> usize {
        self.inputs_data.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs_data.len()
    }

    pub fn is_input(&self) -> bool {
        self.info
            .source
            .primitive()
            .is_some_and(Primitive::is_input)
    }

    pub fn is_output(&self) -> bool {
        self.info
            .source
            .primitive()
            .is_some_and(Primitive::is_output)
    }

    pub fn add_interaction(&mut self, interaction: UserInteraction) {
        if self.user_interaction.is_none() {
            self.user_interaction = Some(Vec::new());
        }
        self.user_interaction.as_mut().unwrap().push(interaction);
    }

    pub const fn with_pos(mut self, pos: Pos2) -> Self {
        self.pos = pos;
        self
    }

    pub const fn with_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn update_comp_info(&mut self, info: ComponentInfo) {
        self.inputs_data = info.inputs.iter().map(|io| Data::new(0, io.size)).collect();
        self.outputs_data = info
            .outputs
            .iter()
            .map(|io| Data::new(0, io.size))
            .collect();
        self.info = info;
    }

    pub fn from_comp_info(info: ComponentInfo) -> Self {
        let inputs_data = info.inputs.iter().map(|io| Data::new(0, io.size)).collect();
        let outputs_data = info
            .outputs
            .iter()
            .map(|io| Data::new(0, io.size))
            .collect();

        Self {
            pos: Pos2::default(),
            id: 0,
            info,
            inputs_data,
            outputs_data,
            user_interaction: None,
        }
    }

    pub fn build_primitive(
        &mut self,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        if self.info.source.primitive().is_none() {
            return Err(BoardBuildError::PrimitiveNotSpecified);
        }

        let id = *last_id;
        *last_id += 1;
        self.id = id;

        Ok((
            IdMap::new(
                id,
                self.info.name.clone(),
                CompSource::Prim(self.info.source.primitive().cloned().unwrap()),
            ),
            Component {
                id,
                name: Some(self.info.name.clone()),
                inputs: self.input_count(),
                outputs: self.output_count(),
                sub: None,
                extra: ExtraInfo {
                    id: self.id,
                    primitive: self.info.source.primitive().cloned(),
                },
            },
        ))
    }

    pub fn build_component(
        &mut self,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        if self.info.source.primitive().is_some() {
            return self.build_primitive(last_id);
        }

        let source = self
            .info
            .source
            .local()
            .cloned()
            .ok_or(BoardBuildError::SourceNotSpecified)?;

        let mut board = Board::load(&source)?;

        board.build_component(self.info.source.clone(), last_id)
    }
}
