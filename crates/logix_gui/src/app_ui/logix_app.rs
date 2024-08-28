use crate::app_ui::folder_tree::Folder;
use egui::FontId;
use egui_notify::Toasts;
use std::path::PathBuf;

use super::{
    app_config::AppSettings,
    app_data::AppData,
    app_state::{AppState, LeftPannelState},
    board_editing::BoardEditing,
    shortcuts,
};

pub struct LogixApp {
    pub folder: Option<Folder>,
    pub selected_file: Option<PathBuf>,
    pub board_tabs: Vec<BoardEditing>,
    pub current_tab: usize,
    pub toasts: Toasts,
    pub state: AppState,

    pub settings: AppSettings,
    pub data: AppData,

    pub new_project_folder: String,
    pub new_project_name: String,
}

impl Default for LogixApp {
    fn default() -> Self {
        Self {
            folder: None,
            selected_file: None,
            board_tabs: vec![BoardEditing::default()],
            current_tab: 0,
            toasts: Toasts::new().with_default_font(FontId::proportional(10.0)),
            state: AppState::default(),
            settings: AppSettings::default(),
            data: AppData::default(),
            new_project_folder: String::default(),
            new_project_name: String::default(),
        }
    }
}

impl LogixApp {
    pub fn from_folder(path: &PathBuf) -> Result<Self, std::io::Error> {
        let folder = Folder::from_pathbuf(path)?;
        let mut app = Self {
            folder: Some(folder),
            state: AppState::OnProject(LeftPannelState::Folders),
            ..Default::default()
        };
        app.load_config_and_data();
        app.try_load_folder(path)?;
        Ok(app)
    }

    fn draw_app(&mut self, ctx: &egui::Context) {
        match &mut self.state {
            AppState::OnWelcome => {
                self.draw_welcome_page(ctx);
            }
            AppState::CreatingNewProject { folder: _, name: _ } => {
                self.draw_new_project(ctx);
            }
            AppState::OnProject(_) => {
                if self.folder.is_none() {
                    self.state = AppState::OnWelcome;
                    return;
                }
                ctx.input_mut(|input| {
                    if input.consume_shortcut(&shortcuts::SAVE) {
                        self.save_current_board();
                    }
                    if input.consume_shortcut(&shortcuts::RUN) {
                        self.run_current_sim();
                    }
                    if input.consume_shortcut(&shortcuts::STOP) {
                        self.stop_current_sim();
                    }
                });

                self.top_panel(ctx);
                self.left_panel(ctx);
                self.draw_tabs(ctx);
                self.status_bar(ctx);
                self.board_editing_mut().show(ctx);
            }
        }
        self.toasts.show(ctx);
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_config_and_data();
        ctx.set_pixels_per_point(self.settings.zoom);
        ctx.style_mut(|style| {
            style.visuals.button_frame = false;
        });

        self.draw_app(ctx);
    }
}
