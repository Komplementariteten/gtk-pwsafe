use rs_pwsafe::PwFile;
use gtk::prelude::*;
use gtk::glib::{Variant, FromVariant, ToVariant};


#[derive(Variant)]
pub struct PwStore {
    pf: Option<PwFile>
}