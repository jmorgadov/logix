use std::path::{Path, PathBuf};

use egui::Pos2;
use logix_core::component::{Component, Conn, PortAddr, SubComponent};
use logix_sim::primitives::primitive::{ExtraInfo, Primitive};
use serde::{Deserialize, Serialize};

use crate::app_ui::id_map::IdMap;

use super::{
    super::errors::{
        BoardBuildError, LoadBoardError, LoadComponentError, OpenBoardError, ReloadComponentsError,
        SaveBoardError,
    },
    board_comp::BoardComponent,
    board_conn::BoardConnection,
    comp_info::ComponentInfo,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IOInfo {
    pub idx: usize,
    pub name: String,
}

impl IOInfo {
    pub const fn new(idx: usize, name: String) -> Self {
        Self { idx, name }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Board {
    pub name: String,

    pub components: Vec<BoardComponent>,

    pub deps: Vec<PathBuf>,

    pub inputs: Vec<IOInfo>,
    pub outputs: Vec<IOInfo>,

    pub conns_info: Vec<BoardConnection>,

    pub in_addrs: Vec<(usize, PortAddr)>,
    pub out_addrs: Vec<PortAddr>,
}

impl Board {
    pub fn from_comp_info(comp: &ComponentInfo) -> Self {
        Self::load(comp.source.as_ref().unwrap()).unwrap()
    }

    pub fn build_component(
        &mut self,
        source: Option<PathBuf>,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        let sub_comps: Vec<(IdMap, Component<ExtraInfo>)> = self
            .components
            .iter_mut()
            .map(|bc| bc.info.build_component(last_id))
            .collect::<Result<Vec<_>, _>>()?;

        let id = *last_id;
        *last_id += 1;
        let id_map = IdMap::from_children(
            id,
            self.name.clone(),
            source,
            sub_comps.iter().map(|(m, _)| m.clone()).collect(),
        );

        let sub = SubComponent {
            components: sub_comps.into_iter().map(|(_, c)| c).collect(),
            connections: self.conns_info.iter().map(|info| info.conn).collect(),
            in_addrs: self.in_addrs.clone(),
            out_addrs: self.out_addrs.clone(),
        };

        Ok((
            id_map,
            Component {
                id,
                name: Some(self.name.clone()),
                inputs: self.inputs.len(),
                outputs: self.outputs.len(),
                sub: Some(sub),
                extra: ExtraInfo {
                    id: 0,
                    primitive: None,
                },
            },
        ))
    }

    pub fn board_comp(&self, id: usize, source: Option<PathBuf>) -> BoardComponent {
        BoardComponent {
            pos: Pos2::default(),
            info: ComponentInfo {
                id,
                name: self.name.clone(),
                source,
                primitive: None,
                inputs_name: self.inputs.iter().map(|input| input.name.clone()).collect(),
                outputs_name: self
                    .outputs
                    .iter()
                    .map(|output| output.name.clone())
                    .collect(),
            },
            inputs_data: self
                .inputs
                .iter()
                .map(|input| {
                    assert!(self.components[input.idx]
                        .info
                        .primitive
                        .clone()
                        .is_some_and(|p| p.is_input()));
                    self.components[input.idx].outputs_data[0]
                })
                .collect(),
            outputs_data: self
                .outputs
                .iter()
                .map(|output| {
                    assert!(self.components[output.idx]
                        .info
                        .primitive
                        .clone()
                        .is_some_and(|p| p.is_output()));
                    self.components[output.idx].inputs_data[0]
                })
                .collect(),
        }
    }

    pub fn save(&mut self, path: &PathBuf) -> Result<(), SaveBoardError> {
        let serialized = serde_json::to_string(self)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> Result<Self, LoadBoardError> {
        let serialized = std::fs::read_to_string(path)?;
        let board = serde_json::from_str(&serialized)?;
        Ok(board)
    }

    pub fn open(path: &PathBuf) -> Result<Self, OpenBoardError> {
        let mut board = Self::load(path)?;
        board.reload_imported_components()?;
        Ok(board)
    }

    pub fn update_deps(&mut self) {
        self.deps = self
            .components
            .iter()
            .filter_map(|bc| bc.info.source.clone())
            .collect();

        self.deps.sort();

        let mut i = 1;
        while i < self.deps.len() {
            let absolute_1 = self.deps[i - 1].canonicalize().unwrap();
            let absolute_2 = self.deps[i].canonicalize().unwrap();

            if absolute_1 == absolute_2 {
                self.deps.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn reload_imported_components(&mut self) -> Result<(), ReloadComponentsError> {
        let mut conns_to_remove = vec![];
        for (comp, source) in self
            .components
            .iter_mut()
            .map(|bc| &mut bc.info)
            .filter_map(|c| c.source.clone().map(|source| (c, source)))
        {
            let c = Self::load_comp(comp.id, source)?;
            *comp = c.info;
            let in_count = comp.input_count();
            let out_count = comp.output_count();

            for (i, info) in self.conns_info.iter().enumerate() {
                if info.conn.from.0 == comp.id && info.conn.from.1 >= out_count {
                    conns_to_remove.push(i);
                    continue;
                }
                if info.conn.to.0 == comp.id && info.conn.to.1 >= in_count {
                    conns_to_remove.push(i);
                    continue;
                }
            }
        }

        for i in conns_to_remove.iter().rev() {
            self.remove_conn(*i);
        }
        Ok(())
    }

    pub fn add_comp(&mut self, comp: BoardComponent) {
        self.components.push(comp);

        let idx = self.components.len() - 1;
        match self.components[idx].info.primitive.clone() {
            Some(Primitive::Input { bits: _ }) => {
                self.inputs.push(IOInfo::new(idx, String::default()));
            }
            Some(Primitive::Output { bits: _ }) => {
                self.outputs.push(IOInfo::new(idx, String::default()));
            }
            _ => {}
        }
        self.update_deps();
    }

    pub fn load_comp(id: usize, source: PathBuf) -> Result<BoardComponent, LoadComponentError> {
        let board = Self::load(&source)?;
        Ok(board.board_comp(id, Some(source)))
    }

    pub fn import_comp(
        &mut self,
        id: usize,
        source: &Path,
        pos: Pos2,
    ) -> Result<(), LoadComponentError> {
        let comp = Self::load_comp(id, source.to_path_buf())?.with_pos(pos);
        self.add_comp(comp);
        Ok(())
    }

    pub fn remove_comp(&mut self, idx: usize) {
        let comp = self.components.remove(idx);

        match comp.info.primitive {
            Some(Primitive::Input { bits: _ }) => {
                self.inputs.retain(|input| input.idx != idx);
            }
            Some(Primitive::Output { bits: _ }) => {
                self.outputs.retain(|output| output.idx != idx);
            }
            _ => {}
        }

        // Remove input/output connections to the component
        self.in_addrs.retain(|(i, _)| *i != idx);
        self.out_addrs.retain(|(i, _)| *i != idx);

        let mut i = 0;
        // Update inputs/outputs indices
        while i < self.inputs.len() {
            if self.inputs[i].idx == idx {
                self.inputs.remove(i);
                continue;
            }
            if self.inputs[i].idx > idx {
                self.inputs[i].idx -= 1;
            }
            i += 1;
        }

        i = 0;
        while i < self.outputs.len() {
            if self.outputs[i].idx == idx {
                self.outputs.remove(i);
                continue;
            }
            if self.outputs[i].idx > idx {
                self.outputs[i].idx -= 1;
            }
            i += 1;
        }

        // Update connections
        i = 0;
        while i < self.conns_info.len() {
            let conn = self.conns_info[i].conn;

            // Remove connections related to the component
            if conn.from.0 == idx || conn.to.0 == idx {
                self.conns_info.remove(i);
                continue;
            }

            // Update forward connections indices
            if conn.from.0 > idx {
                self.conns_info[i].conn.from.0 -= 1;
            }
            if conn.to.0 > idx {
                self.conns_info[i].conn.to.0 -= 1;
            }
            i += 1;
        }

        self.update_deps();
    }

    pub fn add_conn(
        &mut self,
        from: usize,
        to: usize,
        from_port: usize,
        to_port: usize,
        points: Vec<Pos2>,
    ) {
        let conn = Conn {
            from: (from, from_port),
            to: (to, to_port),
        };
        self.conns_info.push(BoardConnection { conn, points });

        match &self.components[from].info.primitive {
            Some(Primitive::Input { .. }) => {
                let from_input = self
                    .inputs
                    .iter()
                    .position(|input| input.idx == from)
                    .unwrap();
                self.in_addrs.push((from_input, (to, to_port)));
            }
            Some(Primitive::Output { .. }) => {
                self.out_addrs.push((from, from_port));
            }
            _ => {}
        }
    }

    pub fn remove_conn(&mut self, idx: usize) {
        let info = self.conns_info.remove(idx);

        // Check if connection is an input connection
        if matches!(
            &self.components[info.conn.from.0].info.primitive,
            Some(Primitive::Input { .. })
        ) {
            let mut i = 0;
            while i < self.in_addrs.len() {
                if self.in_addrs[i].0 == info.conn.from.0 && self.in_addrs[i].1 == info.conn.to {
                    self.in_addrs.remove(i);
                    break;
                }
                i += 1;
            }
        }

        // Check if connection is an output connection
        if matches!(
            &self.components[info.conn.to.0].info.primitive,
            Some(Primitive::Output { .. })
        ) {
            let mut i = 0;
            while i < self.out_addrs.len() {
                if self.out_addrs[i] == info.conn.from {
                    self.out_addrs.remove(i);
                    break;
                }
                i += 1;
            }
        }
    }

    pub fn add_and_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let and_gate = BoardComponent::and_gate(id, in_count).with_pos(pos);
        self.add_comp(and_gate);
    }

    pub fn add_nand_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nand_gate = BoardComponent::nand_gate(id, in_count).with_pos(pos);
        self.add_comp(nand_gate);
    }

    pub fn add_or_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let or_gate = BoardComponent::or_gate(id, in_count).with_pos(pos);
        self.add_comp(or_gate);
    }

    pub fn add_nor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nor_gate = BoardComponent::nor_gate(id, in_count).with_pos(pos);
        self.add_comp(nor_gate);
    }

    pub fn add_xor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let xor_gate = BoardComponent::xor_gate(id, in_count).with_pos(pos);
        self.add_comp(xor_gate);
    }

    pub fn add_not_gate(&mut self, id: usize, pos: Pos2) {
        let not_gate = BoardComponent::not_gate(id).with_pos(pos);
        self.add_comp(not_gate);
    }

    pub fn add_const_high_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = BoardComponent::const_high_gate(id).with_pos(pos);
        self.add_comp(const_gate);
    }

    pub fn add_const_low_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = BoardComponent::const_low_gate(id).with_pos(pos);
        self.add_comp(const_gate);
    }

    pub fn add_clock_gate(&mut self, id: usize, pos: Pos2) {
        let clock_gate = BoardComponent::clock_gate(id).with_pos(pos);
        self.add_comp(clock_gate);
    }

    pub fn add_splitter(&mut self, id: usize, bits: u8, pos: Pos2) {
        let splitter = BoardComponent::splitter(id, bits).with_pos(pos);
        self.add_comp(splitter);
    }

    pub fn add_joiner(&mut self, id: usize, bits: u8, pos: Pos2) {
        let joiner = BoardComponent::joiner(id, bits).with_pos(pos);
        self.add_comp(joiner);
    }

    pub fn add_input(&mut self, id: usize, bits: u8, pos: Pos2) {
        let input = BoardComponent::input(id, bits).with_pos(pos);
        self.add_comp(input);
    }

    pub fn add_output(&mut self, id: usize, bits: u8, pos: Pos2) {
        let output = BoardComponent::output(id, bits).with_pos(pos);
        self.add_comp(output);
    }
}

impl ComponentInfo {
    pub fn build_primitive(
        &mut self,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        if self.primitive.is_none() {
            return Err(BoardBuildError::PrimitiveNotSpecified);
        }

        let id = *last_id;
        *last_id += 1;
        self.id = id;

        Ok((
            IdMap::new(id, self.name.clone(), None),
            Component {
                id,
                name: Some(self.name.clone()),
                inputs: self.input_count(),
                outputs: self.output_count(),
                sub: None,
                extra: ExtraInfo {
                    id: self.id,
                    primitive: Some(self.primitive.clone().unwrap()),
                },
            },
        ))
    }

    pub fn build_component(
        &mut self,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        if self.primitive.is_some() {
            return self.build_primitive(last_id);
        }

        let source = self
            .source
            .clone()
            .ok_or(BoardBuildError::SourceNotSpecified)?;

        let mut board = Board::load(&source)?;

        board.build_component(self.source.clone(), last_id)
    }
}
