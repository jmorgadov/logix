use crate::app::{comp_board::ComponentBoard, folder_tree::Folder};
use egui::{emath::TSTransform, Id, Pos2, Rect, Ui};
use logix_core::component::PortAddr;
use logix_sim::Simulator;
use std::{path::PathBuf, thread};

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

    fn update_comp_vals(&mut self) {
        if let None = self.sim {
            return;
        }

        let sim = self.sim.as_mut().unwrap();
        sim.component(|comp| {
            for i in 0..self.current_comp.components.len() {
                let id = self.current_comp.components[i].id;
                let (in_vals, out_vals) = comp.get_status(&[id]);
                self.current_comp.components[i].inputs_data = in_vals;
                self.current_comp.components[i].outputs_data = out_vals;
            }
        });
        thread::sleep(std::time::Duration::from_millis(5));
    }

    pub fn draw_subs(&mut self, ui: &mut Ui, transform: TSTransform, id: Id, rect: Rect) {
        self.update_comp_vals();
        let window_layer = ui.layer_id();
        let mut over_conn: Option<usize> = None;
        let mut i = 0;
        while i < self.current_comp.components.len() {
            let id = egui::Area::new(id.with(("subc", i)))
                .fixed_pos(self.current_comp.comp_pos[i])
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(transform.inverse() * rect);
                    self.draw_comp(ui, i, transform, &mut over_conn);
                })
                .response
                .layer_id;
            ui.ctx().set_transform_layer(id, transform);
            ui.ctx().set_sublayer(window_layer, id);
            i += 1;
        }
        self.over_connection = over_conn;
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.left_panel(ctx);
        self.draw_canvas(ctx);
    }
}
