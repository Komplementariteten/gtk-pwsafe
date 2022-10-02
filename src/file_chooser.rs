use gtk::{ActionBar, ScrolledWindow};

/* pub fn prepare_action_panel() -> ActionBar {
    let bar = ActionBar::builder().height_request(100).build();
    bar.set
} */

pub fn prepare_main_panel() -> ScrolledWindow {
    ScrolledWindow::builder().height_request(400).build()
}
/* #[derive(Default)]
struct FileChooserPanel;

#[glib::object_subclass]
impl ObjectSubclass for FileChooserPanel {
    const NAME: &'static str = "";
    type Type = super::FileChooserPanel;
    type ParentType = super::Actio;
    type Interfaces = ();
    type Instance = ();
    type Class = ();
} */