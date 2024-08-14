mod comp_board;
mod folder_tree;
mod logix_app;

use eframe::egui;
use logix_app::LogixApp;

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        follow_system_theme: true,
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_shown(false)
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "logix_gui",
        options,
        Box::new(|_cc| Ok(Box::<LogixApp>::default())),
    )
}
