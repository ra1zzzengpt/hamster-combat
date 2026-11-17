use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, glib,
};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.example.HamsterCombat4")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hamster Combat 4")
        .build();

    let root = GtkBox::new(Orientation::Vertical, 16);

    root.set_margin_top(24);
    root.set_margin_bottom(24);
    root.set_margin_start(24);
    root.set_margin_end(24);

    let title = Label::new(Some("HAMSTER COMBAT 4"));

    let stats = GtkBox::new(Orientation::Horizontal, 24);
    stats.set_halign(Align::Center);

    let balance = Label::new(Some("Balance: 0 HMSTR"));
    let power = Label::new(Some("Per click: 1"));

    stats.append(&balance);
    stats.append(&power);

    let picture = gtk4::Picture::for_filename(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/sad_homyak.png"
    ));
    picture.set_can_shrink(true);
    picture.set_keep_aspect_ratio(true);
    picture.set_size_request(400, 400);

    let hamster = Button::with_label("HAMSTER");

    hamster.set_size_request(240, 100);
    hamster.set_halign(Align::Center);

    let coins = Rc::new(Cell::new(0_u64));

    let coins_for_handler = Rc::clone(&coins);
    let balance_for_handler = balance.clone();

    hamster.connect_clicked(move |_| {
        let next = coins_for_handler.get() + 1;

        coins_for_handler.set(next);

        balance_for_handler.set_label(&format!("Balance: {next} HMSTR"));
    });

    root.append(&title);
    root.append(&stats);
    root.append(&picture);
    root.append(&hamster);

    window.set_child(Some(&root));
    window.present();
}
