#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gio;
extern crate glib;
extern crate gtk;

use glib::clone;
use gtk::prelude::*;
use gtk::prelude::EntryExt;
use gtk::{Builder, Entry};
use gtk::prelude::GridExt;

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

    let mut app = Rc::new(RefCell::new(App::new(window.clone())));

    //this item used to storage entry fields in order to add it to resulting List of students
    let mut student = Rc::new(RefCell::new(Student {id: "".to_string(), name: "".to_string(), age: 0u8, _id_internal: -1i32}));
    let name_entry: Entry = builder.get_object("entry3").expect("Couldn't get id entry field");
    let id_entry: Entry = builder.get_object("entry1").expect("Couldn't get name entry field");
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

    let open_button: gtk::MenuItem = builder.get_object("imagemenuitem12").expect("Couldn't get read button from menu bar");
    let write_button: gtk::MenuItem =  builder.get_object("imagemenuitem14").expect("Couldn't get write button from menu bar");

    let paned: gtk::Paned = builder.get_object("paned1").expect("Couldn't get paned");
    //let grid: gtk::Grid = builder.get_object("grid1").expect("Couldn't get grid");

    student.borrow_mut().id = "Text".to_string();

    let add_button: gtk::Button = builder.get_object("button1").expect("Couldn't get add button");
    let mut i = Rc::new(Cell::new(1));
    // Adding to grid
    add_button.connect_clicked(clone!(@weak window, @strong i, @strong student, @strong app, @weak builder => move |_| {
        let grid: gtk::Grid = builder.get_object("grid1").expect("Couldn't get grid");
        let entry1 = Entry::new();
        let entry2 = Entry::new();
        let entry3 = Entry::new();
        let delete_button = gtk::Button::new();

        let mut item =
        {
            let borrowed = student.as_ref();
            entry1.set_text(borrowed.borrow().borrow().id.as_str());
            entry2.set_text(borrowed.borrow().borrow().name.as_str());
            entry3.set_text(borrowed.borrow().borrow().age.to_string().as_str());

            Rc::new(RefCell::new(Student {
              id: borrowed.borrow().borrow().id.as_str().to_string(),
              name: borrowed.borrow().borrow().name.as_str().to_string(),
              age: borrowed.borrow().borrow().age,
              _id_internal: app.as_ref().borrow().borrow().idx}))
        };
        app.borrow_mut().storage.add(item.clone());
        app.borrow_mut().idx += 1;

        entry1.connect_changed(clone!(@strong item, @weak entry1 => move |_|
        {
            item.borrow_mut().id = entry1.get_text().expect("Couldn't get text from id entry").to_string();
        }));
        entry2.connect_changed(clone!(@strong item, @weak entry2 => move |_|
        {
            item.borrow_mut().name = entry2.get_text().expect("Couldn't get text from id entry").to_string();
        }));
        entry3.connect_changed(clone!(@strong item, @weak entry3 => move |_|
        {
             if entry3.get_text().expect("Couldn't get text from age entry").to_string().trim().is_empty()
             {
                return;
             }
             let value = entry3.get_text().expect("Couldn't get text from age entry").to_string().trim().parse();
             match value {
             Ok(x) => item.borrow_mut().age = x,
             Err(e) => entry3.set_text(format!("ERROR INPUT {:?}", e).as_str())
             }
        }));
        delete_button.connect_clicked(clone!(@strong item, @weak entry1, @weak entry2, @weak entry3,
         @weak delete_button, @weak app, @weak grid => move |_|
        {
             app.borrow_mut().storage.delete(item.as_ref().borrow().borrow()._id_internal);
             grid.remove(&entry1);
             grid.remove(&entry2);
             grid.remove(&entry3);
             grid.remove(&delete_button);
        }));

        grid.attach(&entry1, 0, (*i).get() /*row*/, 1, 1);
        grid.attach_next_to(&entry2, Some(&entry1),gtk::PositionType::Right,1,1);
        grid.attach_next_to(&entry3, Some(&entry2),gtk::PositionType::Right,1,1);
        grid.attach_next_to(&delete_button, Some(&entry3),gtk::PositionType::Right,1,1);
        (*i).set((*i).get()+1);
        app.borrow_mut().idx = (*i).get() - 1;

        window.show_all();
    }));


    //reader
    open_button.connect_activate(clone!(@weak window, @weak app, @strong i => move |_| {
        // Clear grid
        let grid: gtk::Grid = builder.get_object("grid1").expect("Couldn't get grid");
        loop
        {
            if grid.get_child_at(0,1) != None
            {
                grid.remove_row(1);
            }
            else
            {
                break;
            }
        }
        app.borrow_mut().storage.storage.clear();
        (*i).set(1);

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

            // Reading from file
            app.borrow_mut().storage.read_from_file(filename.as_path());

            // Adding data to ui

            for item in app.borrow_mut().storage.storage.clone()
            {
                let entry1 = Entry::new();
                let entry2 = Entry::new();
                let entry3 = Entry::new();
                let delete_button = gtk::Button::new();

                entry1.set_text(item.as_ref().borrow().borrow().id.as_str());
                entry2.set_text(item.as_ref().borrow().borrow().name.as_str());
                entry3.set_text(item.as_ref().borrow().borrow().age.to_string().as_str());

                entry1.connect_changed(clone!(@strong item, @weak entry1 => move |_|
                    {
                        item.borrow_mut().id = entry1.get_text().expect("Couldn't get text from id entry").to_string();
                    }));
                entry2.connect_changed(clone!(@strong item, @weak entry2 => move |_|
                    {
                        item.borrow_mut().name = entry2.get_text().expect("Couldn't get text from id entry").to_string();
                    }));
                entry3.connect_changed(clone!(@strong item, @weak entry3 => move |_|
                    {
                         if entry3.get_text().expect("Couldn't get text from age entry").to_string().trim().is_empty()
                         {
                            return;
                         }
                         let value = entry3.get_text().expect("Couldn't get text from age entry").to_string().trim().parse();
                         match value {
                         Ok(x) => item.borrow_mut().age = x,
                         Err(e) => entry3.set_text(format!("ERROR INPUT {:?}", e).as_str())
                         }
                    }));

                delete_button.connect_clicked(clone!(@strong item, @weak entry1, @weak entry2, @weak entry3,
                     @weak delete_button, @weak app, @weak grid => move |_|
                    {
                         app.borrow_mut().storage.delete(item.as_ref().borrow().borrow()._id_internal);
                         grid.remove(&entry1);
                         grid.remove(&entry2);
                         grid.remove(&entry3);
                         grid.remove(&delete_button);
                    }));

                grid.attach(&entry1, 0, (*i).get() /*row*/, 1, 1);
                grid.attach_next_to(&entry2, Some(&entry1),gtk::PositionType::Right,1,1);
                grid.attach_next_to(&entry3, Some(&entry2),gtk::PositionType::Right,1,1);
                grid.attach_next_to(&delete_button, Some(&entry3),gtk::PositionType::Right,1,1);
                (*i).set((*i).get()+1);
                //app.borrow_mut().idx = (*i).get() - 1;
            }
        }

        file_chooser.destroy();
        window.show_all();
    }));

    write_button.connect_activate(clone!(@weak window, @weak app => move |_| {
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Save File"),
            Some(&window),
            gtk::FileChooserAction::Save,
        );
        file_chooser.add_buttons(&[
            ("Save", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        if file_chooser.run() == gtk::ResponseType::Ok {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            //let file = File::open(&filename).expect("Couldn't open file");

            app.borrow_mut().storage.write_to_file_choosen(filename.as_path());
            println!("Write success");
        }

        file_chooser.destroy();
    }));

    window.show_all();
}
