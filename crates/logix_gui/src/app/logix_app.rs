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
    pub current_comp: ComponentBoard,
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
            current_comp: Default::default(),
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

    pub fn load_board(&mut self, path: &PathBuf) {
        let comp_res = ComponentBoard::load(path);
        if let Ok(comp) = comp_res {
            self.current_comp = comp;
            self.last_id = self
                .current_comp
                .components
                .iter()
                .map(|c| c.id)
                .max()
                .unwrap()
                + 1;
            self.reset_field();
        }
    }

    pub fn update_comp_vals(&mut self) {
        if let None = self.sim {
            return;
        }

        let sim = self.sim.as_mut().unwrap();
        sim.component(|comp| {
            for i in 0..self.current_comp.components.len() {
                for j in 0..self.current_comp.components[i].input_count() {
                    let (id, idx) = self.current_comp.components[i].inputs_data_idx[j];
                    let data = comp.get_input_status_at(id, idx);
                    self.current_comp.components[i].inputs_data[j] = data;
                }

                for j in 0..self.current_comp.components[i].output_count() {
                    let (id, idx) = self.current_comp.components[i].outputs_data_idx[j];
                    let data = comp.get_output_status_at(id, idx);
                    self.current_comp.components[i].outputs_data[j] = data;
                }
            }
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
