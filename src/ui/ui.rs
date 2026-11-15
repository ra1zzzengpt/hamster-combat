use crate::core::app::HamsterApp;
use crate::ui::main_window::draw_top;

const UP_COST : u64 = 10;
const UP_COUNT : u64 = 1;

impl eframe::App for HamsterApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("header").show(ui, |ui| {
            draw_top(ui);
        });

        egui::Panel::right("right").show(ui, |ui| {

        });
        egui::CentralPanel::default().show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for i in 0..50 {
                        egui::Frame::new()
                            .corner_radius(12)
                            .inner_margin(15)
                            .show(ui, |ui| {
                                ui.heading("Auto Clicker");
                                ui.label("+1 HMSTR per second");
                                ui.button(format!("BUY {}",i));
                            });
                    }
                });
            });
        });
    }
}
