use egui::Ui;
use logix_sim::{flatten::FlattenComponent, Simulator};
use rfd::FileDialog;

use crate::app::{folder_tree::Folder, LogixApp};

impl LogixApp {
    fn file_menu(&mut self, ui: &mut Ui) {
        ui.set_max_width(200.0); // To make sure we wrap long text

        if ui.button("New Board").clicked() {
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Open folder").clicked() {
            let new_folder = FileDialog::new().pick_folder();
            self.folder = Some(Folder::from_pathbuf(&new_folder.unwrap()));
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Save board").clicked() {
            let file = FileDialog::new().save_file();
            if let Some(new_folder) = file {
                let _ = self.current_comp.save(&new_folder);
            }
            ui.close_menu();
        }
        if ui.button("Load board").clicked() {
            let file = FileDialog::new().pick_file();
            if let Some(new_folder) = file {
                self.load_board(&new_folder);
            }
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Exit").clicked() {
            std::process::exit(0);
        }
    }

    pub fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(1.0);
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| self.file_menu(ui));
                ui.menu_button("Sim", |ui| {
                    if ui.button("Start").clicked() {
                        let mut initial_id = 0;
                        let flatten = FlattenComponent::new(
                            self.current_comp.build_component(&mut initial_id).unwrap(),
                        )
                        .unwrap();
                        println!("{:?}", flatten);
                        self.sim = Some(Simulator::new(flatten));
                        self.sim.as_mut().unwrap().start(true);
                        ui.close_menu();
                    }
                    if ui.button("Stop").clicked() {
                        self.sim = None;
                        ui.close_menu();
                    }
                });
            });
            ui.add_space(1.0);
        });
    }
}
