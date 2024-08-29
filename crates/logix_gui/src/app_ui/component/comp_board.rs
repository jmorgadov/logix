use std::path::{Path, PathBuf};

use egui::Pos2;
use logix_core::component::{Component, Conn, PortAddr, SubComponent};
use logix_sim::primitives::primitive::{ExtraInfo, Primitive};
use serde::{Deserialize, Serialize};

use super::{
    super::errors::{
        BoardBuildError, LoadBoardError, LoadComponentError, OpenBoardError, ReloadComponentsError,
        SaveBoardError,
    },
    comp_info::{ComponentInfo, ConnectionInfo},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdMap {
    pub id: usize,
    pub name: String,
    pub source: Option<PathBuf>,
    pub sub_ids: Vec<IdMap>,
}

impl IdMap {
    pub const fn new(id: usize, name: String, source: Option<PathBuf>) -> Self {
        Self {
            id,
            name,
            source,
            sub_ids: vec![],
        }
    }

    pub fn ids(&self) -> Vec<usize> {
        self.sub_ids.iter().map(|map| map.id).collect()
    }

    pub fn from_children(
        id: usize,
        name: String,
        source: Option<PathBuf>,
        children: Vec<Self>,
    ) -> Self {
        Self {
            id,
            name,
            source,
            sub_ids: children,
        }
    }

    pub fn id_walk(&self, id_path: &[usize]) -> Option<&Self> {
        let mut current = self;
        for id in id_path {
            current = current.sub_ids.iter().find(|map| map.id == *id)?;
        }
        Some(current)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentBoard {
    pub name: String,
    pub inputs: usize,
    pub outputs: usize,
    pub comp_pos: Vec<Pos2>,
    pub comp_conns: Vec<ConnectionInfo>,

    pub components: Vec<ComponentInfo>,
    pub deps: Vec<PathBuf>,

    pub inputs_idx: Vec<usize>,
    pub outputs_idx: Vec<usize>,
    pub connections: Vec<Conn>,
    pub in_addrs: Vec<(usize, PortAddr)>,
    pub out_addrs: Vec<PortAddr>,

    pub inputs_name: Vec<String>,
    pub outputs_name: Vec<String>,
}

impl ComponentBoard {
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
            .map(|c| c.build_component(last_id))
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
            connections: self.connections.clone(),
            in_addrs: self.in_addrs.clone(),
            out_addrs: self.out_addrs.clone(),
        };

        Ok((
            id_map,
            Component {
                id,
                name: Some(self.name.clone()),
                inputs: self.inputs,
                outputs: self.outputs,
                sub: Some(sub),
                extra: ExtraInfo {
                    id: 0,
                    primitive: None,
                },
            },
        ))
    }

    pub fn board_info(&self, id: usize, source: Option<PathBuf>) -> ComponentInfo {
        ComponentInfo {
            id,
            name: self.name.clone(),
            source,
            primitive: None,
            inputs_name: self.inputs_name.clone(),
            outputs_name: self.outputs_name.clone(),
            inputs_data: self
                .inputs_idx
                .iter()
                .map(|i| {
                    assert!(self.components[*i]
                        .primitive
                        .clone()
                        .is_some_and(|p| p.is_input()));
                    self.components[*i].outputs_data[0]
                })
                .collect(),
            outputs_data: self
                .outputs_idx
                .iter()
                .map(|i| {
                    assert!(self.components[*i]
                        .primitive
                        .clone()
                        .is_some_and(|p| p.is_output()));
                    self.components[*i].inputs_data[0]
                })
                .collect(),
            inputs_data_idx: self.inputs_idx.iter().map(|_| (0, 0)).collect(),
            outputs_data_idx: self.outputs_idx.iter().map(|_| (0, 0)).collect(),
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
            .filter_map(|c| c.source.clone())
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
            .filter_map(|c| c.source.clone().map(|source| (c, source)))
        {
            let c = Self::load_comp(comp.id, source)?;
            *comp = c;
            let in_count = comp.input_count();
            let out_count = comp.output_count();

            for (i, conn) in self.connections.iter().enumerate() {
                if conn.from.0 == comp.id && conn.from.1 >= out_count {
                    conns_to_remove.push(i);
                    continue;
                }
                if conn.to.0 == comp.id && conn.to.1 >= in_count {
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

    pub fn add_comp(&mut self, subc: ComponentInfo, pos: Pos2) {
        self.components.push(subc);
        self.comp_pos.push(pos);

        if let Some(prim) = self.components.last().unwrap().primitive.clone() {
            match prim {
                Primitive::Input { bits: _ } => {
                    self.inputs += 1;
                    self.inputs_idx.push(self.components.len() - 1);
                    self.inputs_name.push(String::default());
                }
                Primitive::Output { bits: _ } => {
                    self.outputs += 1;
                    self.outputs_idx.push(self.components.len() - 1);
                    self.outputs_name.push(String::default());
                }
                _ => {}
            }
        }
        self.update_deps();
    }

    pub fn load_comp(id: usize, source: PathBuf) -> Result<ComponentInfo, LoadComponentError> {
        let board = Self::load(&source)?;
        Ok(board.board_info(id, Some(source)))
    }

    pub fn import_comp(
        &mut self,
        id: usize,
        source: &Path,
        pos: Pos2,
    ) -> Result<(), LoadComponentError> {
        let comp = Self::load_comp(id, source.to_path_buf())?;
        self.add_comp(comp, pos);
        Ok(())
    }

    pub fn remove_comp(&mut self, idx: usize) {
        let comp = self.components.remove(idx);
        self.comp_pos.remove(idx);

        if let Some(prim) = comp.primitive {
            match prim {
                Primitive::Input { bits: _ } => {
                    self.inputs -= 1;
                    self.inputs_idx
                        .iter()
                        .position(|&x| x == idx)
                        .map(|i| self.inputs_idx.remove(i));
                }
                Primitive::Output { bits: _ } => {
                    self.outputs -= 1;
                    self.outputs_idx
                        .iter()
                        .position(|&x| x == idx)
                        .map(|i| self.outputs_idx.remove(i));
                }
                _ => {}
            }
        }

        // Remove input/output connections to the component
        self.in_addrs.retain(|(i, _)| *i != idx);
        self.out_addrs.retain(|(i, _)| *i != idx);

        let mut i = 0;
        // Update inputs/outputs indices
        while i < self.inputs_idx.len() {
            if self.inputs_idx[i] == idx {
                self.inputs_idx.remove(i);
                continue;
            }
            if self.inputs_idx[i] > idx {
                self.inputs_idx[i] -= 1;
            }
            i += 1;
        }

        i = 0;
        while i < self.outputs_idx.len() {
            if self.outputs_idx[i] == idx {
                self.outputs_idx.remove(i);
                continue;
            }
            if self.outputs_idx[i] > idx {
                self.outputs_idx[i] -= 1;
            }
            i += 1;
        }

        // Update connections
        i = 0;
        while i < self.connections.len() {
            let conn = self.connections[i];

            // Remove connections related to the component
            if conn.from.0 == idx || conn.to.0 == idx {
                self.connections.remove(i);
                self.comp_conns.remove(i);
                continue;
            }

            // Update forward connections indices
            if conn.from.0 > idx {
                self.connections[i].from.0 -= 1;
            }
            if conn.to.0 > idx {
                self.connections[i].to.0 -= 1;
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
        self.connections.push(Conn {
            from: (from, from_port),
            to: (to, to_port),
        });
        self.comp_conns.push(ConnectionInfo { points });

        if let Some(prim) = &self.components[from].primitive {
            if prim.is_input() {
                let from_input = self.inputs_idx.iter().position(|&x| x == from).unwrap();
                self.in_addrs.push((from_input, (to, to_port)));
            }
        }

        if let Some(prim) = &self.components[to].primitive {
            if prim.is_output() {
                self.out_addrs.push((from, from_port));
            }
        }
    }

    pub fn remove_conn(&mut self, idx: usize) {
        let conn = self.connections[idx];
        self.connections.remove(idx);
        self.comp_conns.remove(idx);

        // Check if connection is an input connection
        if let Some(prim) = &self.components[conn.from.0].primitive {
            if prim.is_input() {
                let mut i = 0;
                while i < self.in_addrs.len() {
                    if self.in_addrs[i].0 == conn.from.0 && self.in_addrs[i].1 == conn.to {
                        self.in_addrs.remove(i);
                        break;
                    }
                    i += 1;
                }
            }
        }

        // Check if connection is an output connection
        if let Some(prim) = &self.components[conn.to.0].primitive {
            if prim.is_output() {
                let mut i = 0;
                while i < self.out_addrs.len() {
                    if self.out_addrs[i] == conn.from {
                        self.out_addrs.remove(i);
                        break;
                    }
                    i += 1;
                }
            }
        }
    }

    pub fn add_and_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let and_gate = ComponentInfo::and_gate(id, in_count);
        self.add_comp(and_gate, pos);
    }

    pub fn add_nand_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nand_gate = ComponentInfo::nand_gate(id, in_count);
        self.add_comp(nand_gate, pos);
    }

    pub fn add_or_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let or_gate = ComponentInfo::or_gate(id, in_count);
        self.add_comp(or_gate, pos);
    }

    pub fn add_nor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nor_gate = ComponentInfo::nor_gate(id, in_count);
        self.add_comp(nor_gate, pos);
    }

    pub fn add_xor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let xor_gate = ComponentInfo::xor_gate(id, in_count);
        self.add_comp(xor_gate, pos);
    }

    pub fn add_not_gate(&mut self, id: usize, pos: Pos2) {
        let not_gate = ComponentInfo::not_gate(id);
        self.add_comp(not_gate, pos);
    }

    pub fn add_const_high_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = ComponentInfo::const_high_gate(id);
        self.add_comp(const_gate, pos);
    }

    pub fn add_const_low_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = ComponentInfo::const_low_gate(id);
        self.add_comp(const_gate, pos);
    }

    pub fn add_clock_gate(&mut self, id: usize, pos: Pos2) {
        let clock_gate = ComponentInfo::clock_gate(id);
        self.add_comp(clock_gate, pos);
    }

    pub fn add_splitter(&mut self, id: usize, bits: u8, pos: Pos2) {
        let splitter = ComponentInfo::splitter(id, bits);
        self.add_comp(splitter, pos);
    }

    pub fn add_joiner(&mut self, id: usize, bits: u8, pos: Pos2) {
        let joiner = ComponentInfo::joiner(id, bits);
        self.add_comp(joiner, pos);
    }

    pub fn add_input(&mut self, id: usize, bits: u8, pos: Pos2) {
        let input = ComponentInfo::input(id, bits);
        self.add_comp(input, pos);
    }

    pub fn add_output(&mut self, id: usize, bits: u8, pos: Pos2) {
        let output = ComponentInfo::output(id, bits);
        self.add_comp(output, pos);
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

        self.inputs_data_idx = (0..self.inputs_name.len()).map(|i| (id, i)).collect();
        self.outputs_data_idx = (0..self.outputs_name.len()).map(|i| (id, i)).collect();

        Ok((
            IdMap::new(id, self.name.clone(), None),
            Component {
                id,
                name: Some(self.name.clone()),
                inputs: self.inputs_data.len(),
                outputs: self.outputs_data.len(),
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

        let mut board = ComponentBoard::load(&source)?;

        let res = board.build_component(self.source.clone(), last_id);

        for (idx, (to, to_port)) in &board.in_addrs {
            self.inputs_data_idx[*idx] = board.components[*to].inputs_data_idx[*to_port];
        }

        for (i, (from, from_port)) in board.out_addrs.iter().enumerate() {
            self.outputs_data_idx[i] = board.components[*from].outputs_data_idx[*from_port];
        }

        res
    }
}
