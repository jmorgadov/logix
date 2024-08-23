use egui::{KeyboardShortcut, Ui};
use rfd::FileDialog;

use crate::app_ui::{
    logix_app::LogixApp,
    shortcuts::{shortcut_string, RUN, SAVE, STOP},
};

impl LogixApp {
    fn named_cmd_shorcut(cmd: &str, shortcut: KeyboardShortcut) -> String {
        format!("{} ({})", cmd, shortcut_string(shortcut))
    }
    fn file_menu(&mut self, ui: &mut Ui) {
        ui.set_max_width(200.0); // To make sure we wrap long text

        if ui.button("New Board").clicked() {
            self.new_board();
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Open folder").clicked() {
            let new_folder = FileDialog::new().pick_folder();
            if let Some(new_folder) = new_folder {
                let _ = self.try_load_folder(&new_folder);
            }
            ui.close_menu();
        }
        ui.separator();
        if ui
            .button(Self::named_cmd_shorcut("Save board", SAVE))
            .clicked()
        {
            self.save_current_board();
            ui.close_menu();
        }
        if ui.button("Load board").clicked() {
            let mut file = FileDialog::new();
            if let Some(folder) = &self.folder {
                file = file.set_directory(folder.current_path.clone());
            }
            if let Some(new_file) = file.pick_file() {
                if self.load_board(&new_file).is_ok() {
                    self.selected_file = Some(new_file);
                }
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
                    if ui.button(Self::named_cmd_shorcut("Start", RUN)).clicked() {
                        self.run_current_sim();
                        ui.close_menu();
                    }
                    if ui.button(Self::named_cmd_shorcut("Stop", STOP)).clicked() {
                        self.stop_current_sim();
                        ui.close_menu();
                    }
                });
            });
            ui.add_space(1.0);
        });
    }
}
