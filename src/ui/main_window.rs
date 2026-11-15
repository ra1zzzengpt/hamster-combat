
pub fn draw_top(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("HAMSTER COMBAT 4");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("exit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ui.label("alpha v0.1")
        })
    });
}

fn draw_main_panel(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Hamster Combat 4");
        ui.label("The legendary comeback");
        ui.columns(2, |columns| {
            columns[0].vertical_centered(|ui| {
                ui.label(egui::RichText::new("BALANCE:").size(28.0).strong());
                ui.label(egui::RichText::new(format!("{} HMSTR",self.coins())).size(28.0).strong().color(egui::Color32::GOLD));
            });
            columns[1].vertical_centered(|ui| {
                ui.label(format!("PER CLICK\n {}", self.per_click()));
            })
        });
        if ui.add(egui::Image::new("file://assets/sad_homyak.png").max_size(egui::vec2(200.0,200.0)).sense(egui::Sense::CLICK)).clicked() {
            self.tap();
        }
        ui.add_space(20.0);
        let can_upgrade = self.coins() >= crate::ui::ui::UP_COST;
        if ui.add_enabled(can_upgrade,egui::Button::new("UPGRADE\n +1 HMSTR per click - Cost: 10"),).clicked() {
            if self.coins() >= crate::ui::ui::UP_COST {
                self.buy(crate::ui::ui::UP_COST);
                self.upgrade_per_click();
            }
        }
}