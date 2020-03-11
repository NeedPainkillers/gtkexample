#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Grid};
use gtk::MenuItem;
use gtk::prelude::GridExt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;

use std::env::args;
use std::cell::{RefCell, Cell};
use std::rc::Rc;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("form.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").expect("Couldn't get window");
    window.set_application(Some(application));

    let open_button: gtk::MenuItem = builder.get_object("imagemenuitem12").expect("Couldn't get open button from menu bar");

    let paned: gtk::Paned = builder.get_object("paned1").expect("Couldn't get paned");
    //let grid: gtk::Grid = builder.get_object("grid1").expect("Couldn't get grid");

    let add_button: gtk::Button = builder.get_object("button1").expect("Couldn't get add button");



    let mut i = Rc::new(Cell::new(1));
    add_button.connect_clicked(clone!(@weak window, @strong i => move |_| {
    //TODO: read from libs binded to entry fields and add result to LinkedList which will be written to file
        let grid: gtk::Grid = builder.get_object("grid1").expect("Couldn't get grid");
        let entry1 = gtk::Entry::new();
        let entry2 = gtk::Entry::new();
        let entry3 = gtk::Entry::new();

        grid.attach(&entry1, 0, (*i).get() /*row*/, 1, 1);
        grid.attach_next_to(&entry2, Some(&entry1),gtk::PositionType::Right,1,1);
        grid.attach_next_to(&entry3, Some(&entry2),gtk::PositionType::Right,1,1);
        (*i).set((*i).get()+1);

        window.show_all();
    }));



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
