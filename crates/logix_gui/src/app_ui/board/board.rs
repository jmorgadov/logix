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
    board_io::BoardIO,
    comp_info::{ComponentInfo, IOInfo},
    CompSource,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Board {
    pub name: String,
    pub deps: Vec<PathBuf>,
    pub components: Vec<BoardComponent>,
    pub conns: Vec<BoardConnection>,
    pub inputs: Vec<BoardIO>,
    pub outputs: Vec<BoardIO>,

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
            .map(|bc| bc.build_component(last_id))
            .collect::<Result<Vec<_>, _>>()?;

        let (ids, sub_comps) = sub_comps.into_iter().unzip();

        let id = *last_id;
        *last_id += 1;
        let id_map = IdMap::from_children(id, self.name.clone(), source, ids);

        let in_addrs = self
            .conns
            .iter()
            .filter_map(|conn| match self.components[conn.conn.from.0].info.source {
                CompSource::Prim(Primitive::Input { .. }) => {
                    let pos = self
                        .inputs
                        .iter()
                        .position(|io| io.idx == conn.conn.from.0)?;
                    Some((pos, conn.conn.to))
                }
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
            id,
            info: ComponentInfo {
                name: self.name.clone(),
                description: None,
                source,
                inputs: self
                    .inputs
                    .iter()
                    .map(|io| {
                        IOInfo::new(
                            io.name.clone(),
                            self.components[io.idx].outputs_data[0].size,
                        )
                    })
                    .collect(),
                outputs: self
                    .outputs
                    .iter()
                    .map(|io| {
                        IOInfo::new(io.name.clone(), self.components[io.idx].inputs_data[0].size)
                    })
                    .collect(),
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
            user_interaction: None,
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
            .filter_map(|c| c.info.source.local().cloned().map(|source| (c, source)))
        {
            let c = Self::load_comp(comp.id, source)?;
            *comp = c.with_pos(comp.pos).with_id(comp.id);
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
}
