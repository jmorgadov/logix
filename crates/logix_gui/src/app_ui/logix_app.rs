use crate::app_ui::folder_tree::Folder;
use egui_notify::Toasts;
use std::{path::PathBuf, time::Instant};

use super::{
    app_config::AppSettings,
    app_data::AppData,
    app_state::{AppState, LeftPannelState},
    board_editing::BoardEditing,
    library::Library,
    shortcuts,
};

const MAX_FPS: f32 = 60.0;

pub struct LogixApp {
    pub folder: Folder,
    pub selected_file: Option<PathBuf>,
    pub board_tabs: Vec<BoardEditing>,
    pub current_tab: usize,
    pub toasts: Toasts,
    pub state: AppState,

    pub render_time: f64,
    pub library: Library,
    pub settings: AppSettings,
    pub data: AppData,
}

impl std::default::Default for LogixApp {
    fn default() -> Self {
        let mut app = Self {
            folder: Folder::default(),
            selected_file: Option::default(),
            board_tabs: Vec::default(),
            current_tab: 0,
            render_time: 0.0,
            toasts: Toasts::default(),
            state: AppState::OnWelcome,
            library: Library::default(),
            settings: AppSettings::default(),
            data: AppData::default(),
        };
        app.load_app();
        app
    }
}

impl LogixApp {
    pub fn from_folder(path: &PathBuf) -> Result<Self, std::io::Error> {
        let folder = Folder::from_pathbuf(path)?;
        let mut app = Self {
            folder,
            state: AppState::OnProject(LeftPannelState::Folders),
            ..Default::default()
        };
        app.load_app();
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
                if !self.folder.is_loaded() {
                    self.state = AppState::OnWelcome;
                    return;
                }

                ctx.input_mut(|input| {
                    if input.consume_shortcut(&shortcuts::SAVE_AS) && self.exist_active_board() {
                        self.save_current_board_as();
                    }
                    if input.consume_shortcut(&shortcuts::SAVE) && self.exist_active_board() {
                        self.save_current_board();
                    }
                    if input.consume_shortcut(&shortcuts::RUN_SIM) && self.exist_active_board() {
                        self.run_current_sim();
                    }
                    if input.consume_shortcut(&shortcuts::PAUSE_RESUME_SIM)
                        && self.exist_active_board()
                    {
                        self.pause_resume_current_sim();
                    }
                    if input.consume_shortcut(&shortcuts::STOP_SIM) && self.exist_active_board() {
                        self.stop_current_sim();
                    }
                    if input.consume_shortcut(&shortcuts::NEW_BOARD) {
                        self.new_board();
                    }

                    if input.consume_shortcut(&shortcuts::UNDO) && self.exist_active_board() {
                        self.board_editing_mut().board.undo();
                    }
                    if input.consume_shortcut(&shortcuts::REDO) && self.exist_active_board() {
                        self.board_editing_mut().board.redo();
                    }
                });

                self.top_panel(ctx);
                self.left_panel(ctx);
                self.draw_tabs(ctx);
                self.status_bar(ctx);

                if self.board_tabs.is_empty() {
                    Self::empty_ui(ctx);
                } else {
                    self.board_editing_mut().show(ctx);
                }
            }
        }
        self.toasts.show(ctx);
    }
}

impl eframe::App for LogixApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        ctx.set_pixels_per_point(self.settings.zoom);
        ctx.style_mut(|style| {
            style.visuals.button_frame = false;
        });

        self.draw_app(ctx);
        let elapsed = now.elapsed();
        if elapsed.as_secs_f32() < 1.0 / MAX_FPS {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / MAX_FPS) - elapsed);
        }
        let after = now.elapsed();
        self.render_time = after.as_secs_f64();
    }
}
