#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Grid};
use gtk::MenuItem;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;

use std::env::args;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("form.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").expect("Couldn't get window");
    window.set_application(Some(application));

    let open_button: gtk::MenuItem = builder.get_object("imagemenuitem12").expect("Couldn't get open button from menu bar");

    let paned: gtk::Paned = builder.get_object("paned1").expect("Couldn't get paned");

    open_button.connect_activate(clone!(@weak window => move |_| {
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
        );
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        if file_chooser.run() == gtk::ResponseType::Ok {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let file = File::open(&filename).expect("Couldn't open file");

            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            //text_view
            //    .get_buffer()
            //    .expect("Couldn't get window")
            //    .set_text(&contents);
        }

        file_chooser.destroy();
    }));

    window.show_all();
}
