use std::path::PathBuf;

use egui::{emath::TSTransform, Pos2};
use logix_core::component::PortAddr;
use logix_sim::{flatten::FlattenComponent, Simulator};

use super::comp_board::ComponentBoard;

#[derive(Default)]
pub struct BoardEditing {
    pub board: ComponentBoard,
    pub file: Option<PathBuf>,
    pub transform: TSTransform,
    pub next_id: usize,
    pub new_conn: Option<(PortAddr, Vec<Pos2>)>,
    pub last_click_pos: Pos2,
    pub over_connection: Option<usize>,
    pub sim: Option<Simulator>,
}

impl BoardEditing {
    pub fn show(&mut self, ctx: &egui::Context) {
        self.draw_canvas(ctx);
    }

    pub fn is_empty(&self) -> bool {
        self.board.components.is_empty()
    }

    pub fn run_sim(&mut self) {
        let mut initial_id = 0;
        let comp = self.board.build_component(&mut initial_id).unwrap();
        let flatten = FlattenComponent::new(comp).unwrap();
        self.sim = Some(Simulator::new(flatten));
        self.sim.as_mut().unwrap().start(true);
    }

    pub fn stop_sim(&mut self) {
        self.sim = None;
    }

    pub fn update_comp_vals(&mut self) {
        let sim = match self.sim.as_mut() {
            Some(sim) => sim,
            None => return,
        };
        sim.component(|comp| {
            self.board.components.iter_mut().for_each(|board_comp| {
                // Update inputs data
                board_comp
                    .inputs_data
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, data)| {
                        let (id, idx) = board_comp.inputs_data_idx[i];
                        *data = comp.get_input_status_at(id, idx);
                    });

                // Update outputs data
                board_comp
                    .outputs_data
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, data)| {
                        let (id, idx) = board_comp.outputs_data_idx[i];
                        *data = comp.get_output_status_at(id, idx);
                    });
            });
        });
    }
}
