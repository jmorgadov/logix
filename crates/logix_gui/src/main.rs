#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod app_ui;

use std::{env::args, path::PathBuf, str::FromStr};

use crate::app_ui::logix_app::LogixApp;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_shown(false)
            .with_maximized(true),
        ..Default::default()
    };

    let app = if args().len() > 1 {
        let path_arg = args().nth(1).unwrap();
        let path_res = PathBuf::from_str(&path_arg);

        match path_res {
            Ok(path) => LogixApp::from_folder(&path).unwrap_or_else(|err| {
                log::error!("Error loading folder: {}", err);
                LogixApp::default()
            }),
            Err(err) => {
                log::error!("Could not find folder.\n{}\nLoading app as default", err);
                LogixApp::default()
            }
        }
    } else {
        LogixApp::default()
    };

    eframe::run_native("Logix", options, Box::new(|_cc| Ok(Box::new(app))))
}
