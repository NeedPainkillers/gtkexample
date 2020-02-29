extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle, Widget, Entry, Box, Button, Application, ApplicationWindow};
use std::fs::File;
use std::io::prelude::*;

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