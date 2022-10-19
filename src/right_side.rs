use std::cell::{Cell, RefCell};
use gtk::{FileChooserAction, FileChooserDialog, FileFilter, glib, Grid, ResponseType};
use gtk::gio::{Action, File, SimpleAction};
use gtk::glib::{clone};
use gtk::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rs_pwsafe::PwFile;
use crate::pwstore::PwStore;

const SELECT_FILE_ACTION: &str = "SelectFile";
const ENTER_PASSWORD_ACTION: &str = "EnterPassword";

pub struct StoreViewModel {
    pub history: Vec<String>,
    filter: FileFilter,
    file_chooser: FileChooserDialog,
}

impl StoreViewModel {
    pub fn new() -> Self {
        let filter = FileFilter::new();
        filter.add_pattern("*.psafe3");
        filter.set_name(Some("PwSafe v3"));
        // let diag = FileChooserDialog::builder().filter(&filter).modal(false).hide_on_close(true).action(FileChooserAction::Open).modal(true).build();
        let diag = FileChooserDialog::new(
            Some("choose pwsafe"),
            gtk::Window::NONE,
            FileChooserAction::Open,
            &[
                ("Open", gtk::ResponseType::Accept),
                ("Cancel", gtk::ResponseType::Cancel),
            ],
        );
        diag.add_filter(&filter);

        StoreViewModel {
            filter,
            file_chooser: diag,
            history: vec![],
        }
    }

    fn set_pw_file(&self, file: File) -> PwFile {
        let file_base = file.basename().expect("couldn't get basename");
        let mut paths = vec![file_base];
        let mut cur_file = file;

        loop {
            match cur_file.parent() {
                None => break,
                Some(p) => {
                    paths.push(p.basename().expect("couldn't get basename"));
                    cur_file = p;
                }
            }
        }

        paths.reverse();
        let mut path = PathBuf::new();
        for p in paths {
            path.push(p);
        }

        let path_str = path.to_str().expect("couldn't convert path to str");
        match PwFile::open(path_str) {
            Ok(pf) => pf,
            Err(e) => panic!("{:?}", e)
        }
    }


    fn prepare_file_select(self: Arc<Self>, btn: &gtk::LockButton) {
        let file_diag = &self.file_chooser;
        let self_clone = Arc::clone(&self);
        btn.connect_clicked(clone!(@weak file_diag => move | btn | {
            file_diag.show();
        }));

        //let enter_pw_act = SimpleAction::new(ENTER_PASSWORD_ACTION, PwStore::)

        file_diag.connect_response(move |fs, rs| {
            println!("{:?}", rs);
            println!("{:?}", fs);
            if rs == ResponseType::Cancel {
                fs.close();
            }
            if rs == ResponseType::Accept {
                let file = fs.file().expect("couldn't get file");
                let pwfile = self_clone.set_pw_file(file);
            }
        });
    }

    pub fn attach_ui(self: Arc<Self>, target: &gtk::Box) {

        let grid = Grid::builder()
            .name("history-grid")
            .margin_start(8)
            .margin_bottom(8)
            .margin_end(8)
            .margin_top(8)
            .vexpand(true)
            .hexpand(true)
            .build();
        let openbtn = gtk::LockButton::builder()
            .name("choose-file")
            .label("open")
            .text_lock("open")
            .text_unlock("close")
            .visible(true)
            .build();
        grid.attach(&openbtn, 1, 4, 1, 1);

        self.prepare_file_select(&openbtn);

        let history = gtk::ListBox::builder()
            .name("history-list")
            .hexpand(true)
            .vexpand(true)
            .build();
        grid.attach(&history, 0, 0, 2, 3);

        target.append(&grid);
    }
}
