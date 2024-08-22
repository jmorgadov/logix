mod app;

use std::{path::PathBuf, str::FromStr};

use app::LogixApp;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_shown(false)
            .with_maximized(true),
        ..Default::default()
    };

    let app = if std::env::args().len() > 1 {
        let path = std::env::args().nth(1).unwrap();
        let path = PathBuf::from_str(&path);

        match path {
            Ok(path) => LogixApp::from_folder(&path).unwrap_or_else(|err| {
                log::error!("Error loading folder: {}", err);
                LogixApp::default()
            }),
            Err(err) => {
                log::error!("Could not load folder.\n{}\nLoading app as default", err);
                LogixApp::default()
            }
        }
    } else {
        LogixApp::default()
    };

    eframe::run_native("logix_gui", options, Box::new(|_cc| Ok(Box::new(app))))
}
