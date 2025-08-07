mod app;
mod models;
mod persistence;
mod ui;
mod utils;

use app::ScordApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Scord - Contestant Scoring"),
        ..Default::default()
    };

    eframe::run_native(
        "Scord",
        options,
        Box::new(|cc| Ok(Box::new(ScordApp::new(cc)))),
    )
}
