#![windows_subsystem = "windows"]
#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]
#![crate_type = "bin"]
#![allow(dead_code)]


extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

mod form;
use form::*;
mod libs;
use libs::app;

fn main() {
    let application =
        gtk::Application::new(Some("rwform.gtk.example"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });


    application.run(&args().collect::<Vec<_>>());
}

/* old main
fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Sample printer");
        window.set_default_size(350, 70);

        let bx = Box::new(Orientation::Vertical, 10);

        let entry = Entry::new();
        let button = Button::new_with_label("Click me!");

        bx.pack_start(&button, false, false, 0);
        bx.pack_start(&entry, false, false, 0);

        entry.connect_activate(|x| println!("{}",x.get_text().unwrap()));
        button.connect_clicked(move |_| {
            let text = entry.get_text().unwrap();
            println!("{}", text);
            let mut file = File::create("data");
            let mut writer = std::io::BufWriter::new(file.unwrap());
            writer.write_all(text.as_bytes());
            //writer.write_all(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
            //file.write_all(text.as_bytes());
        });


        window.add(&bx);

        window.show_all();
    });

    application.run(&[]);
}
*/