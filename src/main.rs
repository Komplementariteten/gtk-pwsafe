use gtk::Application;
use gtk::prelude::*;

mod left_side;
mod right_side;
mod window;
mod pwstore;

const APP_ID: &str = "org.gtkpwsafe.Main";
const APP_TITLE: &str = "Gtk-Pwsafe";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(window::build_appwindow);
    app.run();
}
