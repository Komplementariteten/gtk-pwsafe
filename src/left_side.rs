use gtk::pango::WrapMode;
use gtk::prelude::*;

pub fn attach(target: &gtk::Box) {
    let info_label = gtk::Label::builder()
        .name("desc")
        .label("Select password file to unlock")
        .wrap_mode(WrapMode::Word)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();
    target.append(&info_label);
}
