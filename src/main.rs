extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle, Widget, Entry, Box, Button, Application, ApplicationWindow};


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
            println!("Clicked!");
            println!("{}", entry.get_text().unwrap());
        });


        window.add(&bx);

        window.show_all();
    });

    application.run(&[]);
}