use std::{path::PathBuf, str::FromStr};

use log::error;
use rfd::FileDialog;

use crate::app_ui::{
    app_state::{AppState, LeftPannelState},
    board_editing::BoardEditing,
    comp_board::ComponentBoard,
    errors::OpenBoardError,
    logix_app::LogixApp,
};

impl LogixApp {
    pub fn default_board_editing(&self) -> BoardEditing {
        let mut board = BoardEditing::default();
        board.file = Some(self.folder.as_ref().unwrap().current_path.clone());
        board.file = board.file.map(|mut p| {
            p.push(PathBuf::from_str("untitled.json").unwrap());
            p
        });
        board
    }
    pub fn board_editing_mut(&mut self) -> &mut BoardEditing {
        if self.board_tabs.is_empty() {
            self.board_tabs.push(self.default_board_editing());
            self.current_tab = 0;
        }
        &mut self.board_tabs[self.current_tab]
    }

    pub fn board_editing(&mut self) -> &BoardEditing {
        if self.board_tabs.is_empty() {
            self.board_tabs.push(self.default_board_editing());
            self.current_tab = 0;
        }
        &self.board_tabs[self.current_tab]
    }

    pub fn set_current_tab(&mut self, idx: usize) {
        assert!(idx < self.board_tabs.len());
        // Only change if the tab is different
        if idx != self.current_tab {
            self.current_tab = idx;
            self.selected_file = self.board_tabs[idx].file.clone();
            self.board_editing_mut()
                .board
                .reload_imported_components()
                .expect("Failed to reload imported components when changing to tab");
        }
    }

    pub fn board(&mut self) -> &ComponentBoard {
        &self.board_editing().board
    }

    pub fn new_board(&mut self) {
        self.board_tabs.push(BoardEditing::default());
        self.current_tab = self.board_tabs.len() - 1;
    }

    pub fn load_board(&mut self, path: &PathBuf) -> Result<(), OpenBoardError> {
        // Check first if it is already open in a tab
        for (i, tab) in self.board_tabs.iter().enumerate() {
            if tab.file == Some(path.clone()) {
                self.set_current_tab(i);
                return Ok(());
            }
        }

        let comp = ComponentBoard::open(path).map_err(|err| {
            self.notify_err(err.to_string());
            err
        })?;

        let next_id = comp
            .components
            .iter()
            .map(|c| c.id)
            .max()
            .unwrap_or_default()
            + 1;

        let b_editing = BoardEditing {
            board: comp,
            file: Some(path.clone()),
            project_folder: self.folder.as_ref().map(|f| f.current_path.clone()),
            next_id,
            ..Default::default()
        };

        if self.board_tabs.len() == 1 && self.board_tabs[0].is_empty() {
            // If there is only one tab and it is empty, replace it
            self.board_tabs[0] = b_editing;
        } else {
            // Otherwise, add a new tab
            self.board_tabs.push(b_editing);
            self.set_current_tab(self.board_tabs.len() - 1);
        }

        Ok(())
    }

    pub fn save_current_board(&mut self) {
        let path = self.board_editing().file.clone();
        if let Some(file_path) = path {
            let res = self.board().save(&file_path);
            self.notify_if_err(res);
            return;
        }
        let mut file = FileDialog::new();
        if let Some(folder) = &self.folder {
            file = file.set_directory(folder.current_path.clone());
        }
        if let Some(new_folder) = file.pick_file() {
            let res = self.board().save(&new_folder);
            self.notify_if_err(res);
        }
    }

    pub fn run_current_sim(&mut self) {
        if let Err(err) = self.board_editing_mut().run_sim() {
            error!("Failed to run simulation: {}", err);
            self.board_editing_mut().stop_sim();
        }
    }

    pub fn stop_current_sim(&mut self) {
        self.board_editing_mut().stop_sim();
        self.state = AppState::OnProject(LeftPannelState::Folders);
    }
}
