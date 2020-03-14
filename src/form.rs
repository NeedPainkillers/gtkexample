#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::prelude::EntryExt;
use gtk::{ApplicationWindow, Builder, Button, Grid, Entry};
use gtk::MenuItem;
use gtk::prelude::GridExt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;

use std::env::args;
use std::cell::{RefCell, Cell};
use std::rc::Rc;

use crate::libs::model::student::*;
use crate::libs::app::app::*;
use std::borrow::Borrow;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("form.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").expect("Couldn't get window");
    window.set_application(Some(application));

    let app = app::new(window.clone());

    //this item used to storage entry fields in order to add it to resulting List of students
    let mut student = Rc::new(RefCell::new(Student {id: "".to_string(), name: "".to_string(), age: 0u8}));
    let name_entry: Entry = builder.get_object("entry1").expect("Couldn't get name entry field");
    let id_entry: Entry = builder.get_object("entry3").expect("Couldn't get id entry field");
    let age_entry: Entry = builder.get_object("entry2").expect("Couldn't get age entry field");

    name_entry.connect_changed(clone!(@strong student, @weak name_entry => move |_|
        {
            student.borrow_mut().name = name_entry.get_text().expect("Couldn't get text from name entry").to_string();
        }));
    id_entry.connect_changed(clone!(@strong student, @weak id_entry => move |_|
        {
            student.borrow_mut().id = id_entry.get_text().expect("Couldn't get text from id entry").to_string();
        }));
    age_entry.connect_changed(clone!(@strong student, @weak age_entry => move |_|
        {
        //TODO: change replacing value at entry with error message somewhere on window
             if age_entry.get_text().expect("Couldn't get text from age entry").to_string().trim().is_empty()
             {
                return;
             }
             let value = age_entry.get_text().expect("Couldn't get text from age entry").to_string().trim().parse();
             match value {
             Ok(x) => student.borrow_mut().age = x,
             Err(e) => age_entry.set_text(format!("ERROR INPUT {:?}", e).as_str())
             }

        }));

    let open_button: gtk::MenuItem = builder.get_object("imagemenuitem12").expect("Couldn't get open button from menu bar");

    let paned: gtk::Paned = builder.get_object("paned1").expect("Couldn't get paned");
    //let grid: gtk::Grid = builder.get_object("grid1").expect("Couldn't get grid");

    student.borrow_mut().id = "Text".to_string();

    let add_button: gtk::Button = builder.get_object("button1").expect("Couldn't get add button");
    let mut i = Rc::new(Cell::new(1));
    add_button.connect_clicked(clone!(@weak window, @strong i, @strong student => move |_| {
    //TODO: read from libs binded to entry fields and add result to LinkedList which will be written to file
        let grid: gtk::Grid = builder.get_object("grid1").expect("Couldn't get grid");
        let entry1 = Entry::new();
        let entry2 = Entry::new();
        let entry3 = Entry::new();

        let item =
        {
            let borrowed = student.as_ref();
            entry1.set_text(borrowed.borrow().borrow().id.as_str());
            entry2.set_text(borrowed.borrow().borrow().name.as_str());
            entry3.set_text(borrowed.borrow().borrow().age.to_string().as_str());
            Rc::new(RefCell::new(Student {id: borrowed.borrow().borrow().id.as_str().to_string(), name: borrowed.borrow().borrow().name.as_str().to_string(), age: borrowed.borrow().borrow().age}))
        };

        entry1.connect_changed(clone!(@strong item, @weak entry1 => move |_|
        {
            item.borrow_mut().id = entry1.get_text().expect("Couldn't get text from id entry").to_string();
        }));

        grid.attach(&entry1, 0, (*i).get() /*row*/, 1, 1);
        grid.attach_next_to(&entry2, Some(&entry1),gtk::PositionType::Right,1,1);
        grid.attach_next_to(&entry3, Some(&entry2),gtk::PositionType::Right,1,1);
        (*i).set((*i).get()+1);

        let mut linked_list = StudentList::new();
        let student1 = Student {id: "0".to_string(), name: "smth".to_string(), age: 18};
        let student2 = Student {id: "1".to_string(), name: "smb".to_string(), age: 19};
        linked_list.add(student1);
        linked_list.add(student2);
        linked_list.write_to_file(&"test".to_string());

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
