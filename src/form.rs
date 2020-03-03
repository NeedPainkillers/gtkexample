#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Grid};

use std::env::args;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("form.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").expect("Couldn't get window");
    window.set_application(Some(application));

    let paned: gtk::Paned = builder.get_object("paned1").expect("Couldn't get paned");

    window.show_all();
}
