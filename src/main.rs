pub mod ui;
pub mod core;

use eframe;
use core::app::HamsterApp;
fn main() {
    eframe::run_native(
        "Hamster Combat 4",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(HamsterApp::new(0, 1)))
        })
    ).unwrap();
}