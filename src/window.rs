use std::sync::Arc;
use gtk::Orientation;
use gtk::prelude::*;

use crate::left_side;
use crate::right_side::*;

pub fn build_appwindow(app: &gtk::Application) {
    let mut mngr = Arc::new(StoreViewModel::new());

    let win = gtk::ApplicationWindow::new(app);
    win.set_title(Some("Gtk-Pwsafe"));
    win.set_default_size(800, 600);

    let base_box = gtk::Box::builder()
        .css_name("main")
        .orientation(Orientation::Horizontal)
        .homogeneous(true)
        .build();
    left_side::attach(&base_box);
    mngr.attach_ui(&base_box);
    win.set_child(Some(&base_box));
    win.show();
}
