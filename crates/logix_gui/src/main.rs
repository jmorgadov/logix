mod app;

use app::LogixApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
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
