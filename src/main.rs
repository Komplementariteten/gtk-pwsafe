mod start;
mod appwindow;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";
const APP_TITLE: &str = "Gtk-Pwsafe";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(|app| {
        // let win = example_win::ExApplicationWindow::new(app);
        let win = start::StartWindow::new(app);
        win.show();
    });

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let action_panel = file_chooser::prepare_main_panel();
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .child(&action_panel)
        .build();

    // Present window
    window.present();
}