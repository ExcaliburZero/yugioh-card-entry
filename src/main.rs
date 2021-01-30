extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, ComboBox, ListStore};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../resources/main.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get main_window");
    window.set_application(Some(application));

    let store = ListStore::new(&[glib::Type::String]);
    store.set(&store.append(), &[0], &[&"hi".to_string()]);
    store.set(&store.append(), &[0], &[&"hello".to_string()]);
    store.set(&store.append(), &[0], &[&"goodbye".to_string()]);

    let combo_box: ComboBox = builder.get_object("set_combo_box").unwrap();
    combo_box.set_model(Some(&store));
    combo_box.set_active(Some(0));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.excaliburzero.yugioh-card-entry.main"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
