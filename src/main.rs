#![windows_subsystem = "windows"]
#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]
#![crate_type = "bin"]
#![allow(dead_code)]


extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use std::env::args;

mod form;
use form::*;
mod libs;

fn main() {
    let application =
        gtk::Application::new(Some("rwform.gtk.example"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}