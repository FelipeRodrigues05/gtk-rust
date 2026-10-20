use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Orientation};

const APP_ID: &str = "com.rodrigues.gtk-rust";
const APP_TITLE: &str = "GTK4 Hyprland Rust";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(20)
        .margin_bottom(20)
        .margin_end(20)
        .margin_start(20)
        .build();

    content_box.set_cursor_from_name(Some("crosshair"));

    let click = gtk::GestureClick::new();
    click.connect_pressed(|_, _, _, _| println!("Clicked"));
    content_box.add_controller(click);

    let button = Button::with_label("Sample Label");
    button.connect_clicked(|btn| {
        btn.set_label("Hello");
    });

    content_box.append(&button);
    button.set_cursor_from_name(Some("pointer"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&content_box)
        .build();

    window.present();
}
