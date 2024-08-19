use crate::app::{comp_board::ComponentBoard, folder_tree::Folder};
use egui::{emath::TSTransform, Pos2};
use logix_core::component::PortAddr;
use logix_sim::Simulator;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum WireDir {
    Horizontal,
    Vertical,
}

impl WireDir {
    pub fn opposite(&self) -> Self {
        match self {
            WireDir::Horizontal => WireDir::Vertical,
            WireDir::Vertical => WireDir::Horizontal,
        }
    }
}

pub struct LogixApp {
    pub folder: Option<Folder>,
    pub selected_file: Option<PathBuf>,
    pub transform: TSTransform,
    pub board: ComponentBoard,
    pub last_id: usize,
    pub new_conn: Option<(PortAddr, Vec<(Pos2, WireDir)>)>,
    pub last_click_pos: Pos2,
    pub over_connection: Option<usize>,
    pub sim: Option<Simulator>,
}

impl Default for LogixApp {
    fn default() -> Self {
        let current_folder = std::env::current_dir();
        let folder = match current_folder {
            Ok(folder) => Some(Folder::from_pathbuf(&folder)),
            Err(_) => None,
        };
        Self {
            folder,
            selected_file: None,
            transform: TSTransform::default(),
            board: Default::default(),
            last_id: 0,
            last_click_pos: Pos2::ZERO,
            new_conn: None,
            over_connection: None,
            sim: None,
        }
    }
}

impl LogixApp {
    fn reset_field(&mut self) {
        self.transform = TSTransform::default();
        self.new_conn = None;
        self.last_click_pos = Pos2::ZERO;
        self.over_connection = None;
        self.sim = None;
    }

    pub fn new_board(&mut self) {
        self.board = Default::default();
        self.last_id = 0;
        self.reset_field();
    }

    pub fn load_board(&mut self, path: &PathBuf) {
        let comp_res = ComponentBoard::load(path);
        if let Ok(comp) = comp_res {
            self.board = comp;
            self.last_id = self
                .board
                .components
                .iter()
                .map(|c| c.id)
                .max()
                .unwrap_or_default()
                + 1;
            self.reset_field();
        }
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

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.left_panel(ctx);
        self.draw_canvas(ctx);
    }
}
