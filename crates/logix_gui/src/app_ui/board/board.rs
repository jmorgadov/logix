use std::path::{Path, PathBuf};

use egui::Pos2;
use logix_core::component::{Component, Conn, SubComponent};
use logix_sim::primitives::primitive::{ExtraInfo, Primitive};
use serde::{Deserialize, Serialize};

use crate::app_ui::id_map::IdMap;

use super::{
    super::errors::{
        BoardBuildError, LoadBoardError, LoadComponentError, OpenBoardError, ReloadComponentsError,
        SaveBoardError,
    },
    board_actions::BoardAction,
    board_comp::BoardComponent,
    board_conn::BoardConnection,
    comp_info::ComponentInfo,
    io_info::IOInfo,
    CompSource,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Board {
    pub name: String,
    pub deps: Vec<PathBuf>,
    pub components: Vec<BoardComponent>,
    pub conns: Vec<BoardConnection>,
    pub inputs: Vec<IOInfo>,
    pub outputs: Vec<IOInfo>,

    #[serde(skip)]
    pub hist: Vec<BoardAction>,
    #[serde(skip)]
    pub hist_idx: Option<usize>,
    #[serde(skip)]
    pub saved_idx: Option<usize>,
}

impl Board {
    pub fn from_comp_info(comp: &ComponentInfo) -> Self {
        Self::load(comp.source.local().unwrap()).unwrap()
    }

    pub fn build_component(
        &mut self,
        source: CompSource,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        let sub_comps: Vec<(IdMap, Component<ExtraInfo>)> = self
            .components
            .iter_mut()
            .map(|bc| bc.info.build_component(last_id))
            .collect::<Result<Vec<_>, _>>()?;

        let (ids, sub_comps) = sub_comps.into_iter().unzip();

        let id = *last_id;
        *last_id += 1;
        let id_map = IdMap::from_children(id, self.name.clone(), source, ids);

        let in_addrs = self
            .conns
            .iter()
            .filter_map(|conn| match self.components[conn.conn.from.0].info.source {
                CompSource::Prim(Primitive::Input { .. }) => Some((conn.conn.from.0, conn.conn.to)),
                _ => None,
            })
            .collect();

        let out_addrs = self
            .conns
            .iter()
            .filter_map(|conn| match self.components[conn.conn.to.0].info.source {
                CompSource::Prim(Primitive::Output { .. }) => Some(conn.conn.from),
                _ => None,
            })
            .collect();

        let sub = SubComponent {
            components: sub_comps,
            connections: self.conns.iter().map(|info| info.conn).collect(),
            in_addrs,
            out_addrs,
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

    pub fn board_comp(&self, id: usize, source: CompSource) -> BoardComponent {
        BoardComponent {
            pos: Pos2::default(),
            info: ComponentInfo {
                id,
                name: self.name.clone(),
                source,
                // primitive: None,
                inputs_name: self.inputs.iter().map(|io| io.name.clone()).collect(),
                outputs_name: self.outputs.iter().map(|io| io.name.clone()).collect(),
            },
            inputs_data: self
                .inputs
                .iter()
                .map(|input| {
                    assert!(self.components[input.idx].is_input());
                    self.components[input.idx].outputs_data[0]
                })
                .collect(),
            outputs_data: self
                .outputs
                .iter()
                .map(|output| {
                    assert!(self.components[output.idx].is_output());
                    self.components[output.idx].inputs_data[0]
                })
                .collect(),
        }
    }

    pub fn save(&mut self, path: &PathBuf) -> Result<(), SaveBoardError> {
        let serialized = serde_json::to_string(self)?;
        std::fs::write(path, serialized)?;
        self.saved_idx = self.hist_idx;
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
            .filter_map(|bc| bc.info.source.local().cloned())
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
            .filter_map(|c| c.source.local().cloned().map(|source| (c, source)))
        {
            let c = Self::load_comp(comp.id, source)?;
            *comp = c.info;
            let in_count = comp.input_count();
            let out_count = comp.output_count();

            for (i, info) in self.conns.iter().enumerate() {
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

        conns_to_remove.sort_unstable();
        for i in conns_to_remove.iter().rev() {
            self.remove_conn(*i);
        }
        Ok(())
    }

    pub fn add_comp(&mut self, comp: BoardComponent) {
        self.execute(BoardAction::add_component(comp));
        self.update_deps();
    }

    pub fn load_comp(id: usize, source: PathBuf) -> Result<BoardComponent, LoadComponentError> {
        let board = Self::load(&source)?;
        Ok(board.board_comp(id, CompSource::Local(source)))
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
        self.execute(BoardAction::remove_component(
            self.components[idx].clone(),
            idx,
        ));
        //
        // // Update inputs/outputs if it was an IO component
        // match comp.info.primitive {
        //     Some(Primitive::Input { bits: _ }) => {
        //         self.inputs.retain(|input| input.idx != idx);
        //     }
        //     Some(Primitive::Output { bits: _ }) => {
        //         self.outputs.retain(|output| output.idx != idx);
        //     }
        //     _ => {}
        // }
        //
        // // Update indices in inputs/outputs according to the removed component
        // self.inputs.iter_mut().for_each(|input| {
        //     if input.idx > idx {
        //         input.idx -= 1;
        //     }
        // });
        // self.outputs.iter_mut().for_each(|output| {
        //     if output.idx > idx {
        //         output.idx -= 1;
        //     }
        // });
        //
        // // Remove connections related to the removed component
        // self.conns
        //     .retain(|conn| conn.conn.from.0 != idx && conn.conn.to.0 != idx);
        //
        // // Update indices in connections according to the removed component
        // self.conns.iter_mut().for_each(|conn| {
        //     if conn.conn.from.0 > idx {
        //         conn.conn.from.0 -= 1;
        //     }
        //     if conn.conn.to.0 > idx {
        //         conn.conn.to.0 -= 1;
        //     }
        // });

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
        self.execute(BoardAction::add_connection(BoardConnection {
            conn: Conn::new(from, from_port, to, to_port),
            points,
        }));
    }

    pub fn remove_conn(&mut self, idx: usize) {
        self.execute(BoardAction::remove_connection(self.conns[idx].clone(), idx));
    }

    pub fn add_and_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let and_gate = BoardComponent::and_gate(in_count).with_pos(pos).with_id(id);
        self.add_comp(and_gate);
    }

    pub fn add_nand_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nand_gate = BoardComponent::nand_gate(in_count)
            .with_pos(pos)
            .with_id(id);
        self.add_comp(nand_gate);
    }

    pub fn add_or_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let or_gate = BoardComponent::or_gate(in_count).with_pos(pos).with_id(id);
        self.add_comp(or_gate);
    }

    pub fn add_nor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let nor_gate = BoardComponent::nor_gate(in_count).with_pos(pos).with_id(id);
        self.add_comp(nor_gate);
    }

    pub fn add_xor_gate(&mut self, id: usize, in_count: usize, pos: Pos2) {
        let xor_gate = BoardComponent::xor_gate(in_count).with_pos(pos).with_id(id);
        self.add_comp(xor_gate);
    }

    pub fn add_not_gate(&mut self, id: usize, pos: Pos2) {
        let not_gate = BoardComponent::not_gate().with_pos(pos).with_id(id);
        self.add_comp(not_gate);
    }

    pub fn add_const_high_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = BoardComponent::const_high_gate().with_pos(pos).with_id(id);
        self.add_comp(const_gate);
    }

    pub fn add_const_low_gate(&mut self, id: usize, pos: Pos2) {
        let const_gate = BoardComponent::const_low_gate().with_pos(pos).with_id(id);
        self.add_comp(const_gate);
    }

    pub fn add_clock_gate(&mut self, id: usize, pos: Pos2) {
        let clock_gate = BoardComponent::clock_gate().with_pos(pos).with_id(id);
        self.add_comp(clock_gate);
    }

    pub fn add_splitter(&mut self, id: usize, bits: u8, pos: Pos2) {
        let splitter = BoardComponent::splitter(bits).with_pos(pos).with_id(id);
        self.add_comp(splitter);
    }

    pub fn add_joiner(&mut self, id: usize, bits: u8, pos: Pos2) {
        let joiner = BoardComponent::joiner(bits).with_pos(pos).with_id(id);
        self.add_comp(joiner);
    }

    pub fn add_input(&mut self, id: usize, bits: u8, pos: Pos2) {
        let input = BoardComponent::input(bits).with_pos(pos).with_id(id);
        self.add_comp(input);
    }

    pub fn add_output(&mut self, id: usize, bits: u8, pos: Pos2) {
        let output = BoardComponent::output(bits).with_pos(pos).with_id(id);
        self.add_comp(output);
    }
}

impl ComponentInfo {
    pub fn build_primitive(
        &mut self,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        if self.source.primitive().is_none() {
            return Err(BoardBuildError::PrimitiveNotSpecified);
        }

        let id = *last_id;
        *last_id += 1;
        self.id = id;

        Ok((
            IdMap::new(
                id,
                self.name.clone(),
                CompSource::Prim(self.source.primitive().cloned().unwrap()),
            ),
            Component {
                id,
                name: Some(self.name.clone()),
                inputs: self.input_count(),
                outputs: self.output_count(),
                sub: None,
                extra: ExtraInfo {
                    id: self.id,
                    primitive: self.source.primitive().cloned(),
                },
            },
        ))
    }

    pub fn build_component(
        &mut self,
        last_id: &mut usize,
    ) -> Result<(IdMap, Component<ExtraInfo>), BoardBuildError> {
        if self.source.primitive().is_some() {
            return self.build_primitive(last_id);
        }

        let source = self
            .source
            .local()
            .cloned()
            .ok_or(BoardBuildError::SourceNotSpecified)?;

        let mut board = Board::load(&source)?;

        board.build_component(self.source.clone(), last_id)
    }
}
