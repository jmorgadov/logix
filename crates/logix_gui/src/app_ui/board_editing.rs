use super::{
    board::{Board, CompSource, UserInteraction},
    errors::SimulationError,
    id_map::IdMap,
};
use egui::{emath::TSTransform, Pos2};
use egui_notify::Toasts;
use log::error;
use logix_core::component::PortAddr;
use logix_sim::{flatten::FlattenComponent, Simulator};
use std::{path::PathBuf, time::Duration};

#[derive(Default)]
pub struct BoardEditing {
    pub board: Board,
    pub project_folder: PathBuf,
    pub file: PathBuf,
    pub transform: TSTransform,
    pub next_id: usize,

    pub new_conn: Option<(PortAddr, Vec<Pos2>)>,
    pub last_click_pos: Pos2,
    pub over_connection: Option<usize>,

    pub sim: Option<Simulator>,
    pub sim_ids: IdMap,
    pub sim_at: Option<(Vec<usize>, Board)>,

    pub toasts: Toasts,
}

impl BoardEditing {
    pub fn show(&mut self, ctx: &egui::Context) {
        self.draw_canvas(ctx);
        self.toasts.show(ctx);
    }

    pub fn notify_err(&mut self, err: impl Into<String>) {
        self.toasts.error(err).set_closable(true);
    }

    #[allow(dead_code)]
    pub fn notify(&mut self, err: impl Into<String>, secs: u64) {
        self.toasts
            .info(err)
            .set_duration(Some(Duration::from_secs(secs)))
            .set_show_progress_bar(false)
            .set_closable(true);
    }

    pub fn is_empty(&self) -> bool {
        self.board.components.is_empty()
    }

    pub fn run_sim(&mut self) -> Result<(), SimulationError> {
        let mut initial_id = 0;
        let (sim_ids, comp) = self
            .board
            .build_component(CompSource::Local(self.file.clone()), &mut initial_id)?;

        if let Some(sub) = comp.sub.as_ref() {
            for (i, subc) in sub.components.iter().enumerate() {
                self.board.components[i].id = subc.id;
            }
        }

        let flatten = FlattenComponent::new(comp)?;
        self.sim = Some(Simulator::new(flatten));
        self.sim.as_mut().unwrap().start(true);
        self.sim_ids = sim_ids;
        Ok(())
    }

    pub fn pause_resume_sim(&mut self) {
        if let Some(sim) = self.sim.as_mut() {
            sim.pause_resume();
        }
    }

    pub fn stop_sim(&mut self) {
        if let Some(sim) = self.sim.as_mut() {
            sim.stop();
        }
        self.sim = None;
        self.sim_at = None;
    }

    pub const fn current_sim_board_ref(&self) -> &Board {
        match self.sim_at.as_ref() {
            Some((_, board)) => board,
            None => &self.board,
        }
    }

    pub fn current_sim_board(&mut self) -> &mut Board {
        match self.sim_at.as_mut() {
            Some((_, board)) => board,
            None => &mut self.board,
        }
    }

    pub fn set_sim_at(&mut self, path: &[usize]) {
        if path.is_empty() {
            self.sim_at = None;
            return;
        }

        let id_map = self.sim_ids.id_walk(path).unwrap();
        if id_map.source.local().is_none() {
            return;
        }

        let id = id_map.id;

        let main_id = self.sim_ids.id;
        if id
            == self
                .sim_at
                .as_ref()
                .map_or(main_id, |(path, _)| *path.last().unwrap())
        {
            return;
        }

        let mut board = Board::load(id_map.source.local().unwrap()).unwrap();
        let ids = self.sim_ids.id_walk(path).unwrap().ids();
        for (i, comp) in board.components.iter_mut().enumerate() {
            comp.id = ids[i];
        }

        self.sim_at = Some((path.to_vec(), board));
    }

    pub fn enter_subc_sim(&mut self, id: usize) {
        if let Some((path, board)) = self.sim_at.as_mut() {
            path.push(id);
            let comp = &board.components.iter().find(|c| c.id == id).unwrap().info;
            let mut new_board = Board::from_comp_info(comp);
            let ids = self.sim_ids.id_walk(path.as_slice()).unwrap().ids();
            for (i, comp) in new_board.components.iter_mut().enumerate() {
                comp.id = ids[i];
            }
            *board = new_board;
        } else {
            let comp = &self
                .board
                .components
                .iter()
                .find(|c| c.id == id)
                .unwrap()
                .info;
            let mut new_board = Board::from_comp_info(comp);
            let ids = self.sim_ids.id_walk(&[id]).unwrap().ids();
            for (i, comp) in new_board.components.iter_mut().enumerate() {
                comp.id = ids[i];
            }
            self.sim_at = Some((vec![id], new_board));
        }
    }

    pub fn update_comp_vals(&mut self) {
        let Some(sim) = self.sim.as_mut() else {
            return;
        };
        let res: Result<(), SimulationError> = sim.state(|state| {
            let (ids, board) = match self.sim_at.as_mut() {
                Some((path, board)) => (Some(path), board),
                None => (None, &mut self.board),
            };
            board.components.iter_mut().try_for_each(|board_comp| {
                if let Some(interactions) = board_comp.user_interaction.as_mut() {
                    let prim_comp = state
                        .comp
                        .get_prim_comp(
                            ids.as_ref().map_or(&[], |ids| ids.as_slice()),
                            Some(board_comp.id),
                        )
                        .map_err(|e| SimulationError::RequestComponentData {
                            comp_name: board_comp.info.name.clone(),
                            comp_id: board_comp.id,
                            err: logix_sim::errors::DataRequestError::InvalidComponentId(e),
                        })?;
                    for interaction in interactions.iter_mut() {
                        match interaction {
                            UserInteraction::ChangeInput(port, data) => {
                                prim_comp.inputs[*port] = *data;
                            }
                            UserInteraction::ChangeOutput(port, data) => {
                                prim_comp.outputs[*port] = *data;
                            }
                        }
                    }
                    let id = prim_comp.id;
                    {
                        // Dropping the reference to prim_comp
                        let _ = prim_comp;
                    }
                    let comp_idx = state.comp.id_to_idx[&id];
                    state.to_upd.push(comp_idx);
                    board_comp.user_interaction = None;
                    return Ok(());
                }

                let (input_datas, output_datas) = state
                    .comp
                    .get_status(
                        ids.as_ref().map_or(&[], |ids| ids.as_slice()),
                        Some(board_comp.id),
                    )
                    .map_err(|e| SimulationError::RequestComponentData {
                        comp_name: board_comp.info.name.clone(),
                        comp_id: board_comp.id,
                        err: e,
                    })?;
                board_comp.inputs_data = input_datas;
                board_comp.outputs_data = output_datas;
                Ok(())
            })
        });

        if let Err(err) = res {
            error!(
                "Error updating component values: {:?}/nEnding simulation",
                err
            );
            self.sim = None;
        }
    }
}
