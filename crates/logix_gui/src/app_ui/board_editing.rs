use super::{component::comp_board::ComponentBoard, errors::SimulationError, id_map::IdMap};
use egui::{emath::TSTransform, Pos2};
use egui_notify::Toasts;
use log::error;
use logix_core::component::PortAddr;
use logix_sim::{flatten::FlattenComponent, Simulator};
use std::path::PathBuf;

#[derive(Default)]
pub struct BoardEditing {
    pub board: ComponentBoard,
    pub project_folder: PathBuf,
    pub file: PathBuf,
    pub transform: TSTransform,
    pub next_id: usize,

    pub new_conn: Option<(PortAddr, Vec<Pos2>)>,
    pub last_click_pos: Pos2,
    pub over_connection: Option<usize>,

    pub sim: Option<Simulator>,
    pub sim_ids: IdMap,
    pub sim_at: Option<(Vec<usize>, ComponentBoard)>,

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

    pub fn is_empty(&self) -> bool {
        self.board.components.is_empty()
    }

    pub fn run_sim(&mut self) -> Result<(), SimulationError> {
        let mut initial_id = 0;
        let (sim_ids, comp) = self
            .board
            .build_component(Some(self.file.clone()), &mut initial_id)?;

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

    pub fn stop_sim(&mut self) {
        self.sim = None;
        self.sim_at = None;
    }

    pub const fn current_sim_board_ref(&self) -> &ComponentBoard {
        match self.sim_at.as_ref() {
            Some((_, board)) => board,
            None => &self.board,
        }
    }

    pub fn current_sim_board(&mut self) -> &mut ComponentBoard {
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
        if id_map.source.is_none() {
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

        let mut board = ComponentBoard::load(id_map.source.as_ref().unwrap()).unwrap();
        let ids = self.sim_ids.id_walk(path).unwrap().ids();
        for (i, comp) in board.components.iter_mut().enumerate() {
            comp.id = ids[i];
        }

        self.sim_at = Some((path.to_vec(), board));
    }

    pub fn enter_subc_sim(&mut self, id: usize) {
        if let Some((path, board)) = self.sim_at.as_mut() {
            path.push(id);
            let comp = board.components.iter().find(|c| c.id == id).unwrap();
            let mut new_board = ComponentBoard::from_comp_info(comp);
            let ids = self.sim_ids.id_walk(path.as_slice()).unwrap().ids();
            for (i, comp) in new_board.components.iter_mut().enumerate() {
                comp.id = ids[i];
            }
            *board = new_board;
        } else {
            let comp = self.board.components.iter().find(|c| c.id == id).unwrap();
            let mut new_board = ComponentBoard::from_comp_info(comp);
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
        let res: Result<(), SimulationError> = sim.component(|comp| {
            let (ids, board) = match self.sim_at.as_mut() {
                Some((path, board)) => (Some(path), board),
                None => (None, &mut self.board),
            };
            board.components.iter_mut().try_for_each(|board_comp| {
                let (input_datas, output_datas) = comp
                    .get_status(
                        ids.as_ref().map_or(&[], |ids| ids.as_slice()),
                        Some(board_comp.id),
                    )
                    .map_err(|e| SimulationError::RequestComponentData {
                        comp_name: board_comp.name.clone(),
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
