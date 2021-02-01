extern crate gio;
extern crate glib;
extern crate gtk;
extern crate yugioh_card_entry;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, ComboBox, ListStore};

use yugioh_card_entry::{APIConfig, CardInfoRequest, YGOProDeckAPI, API};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../resources/main.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder
        .get_object("main_window")
        .expect("Couldn't get main_window");
    window.set_application(Some(application));

    let store = ListStore::new(&[glib::Type::String]);

    let combo_box: ComboBox = builder.get_object("set_combo_box").unwrap();
    combo_box.set_model(Some(&store));
    combo_box.set_active(Some(0));

    window.show_all();

    // Get the card sets and add them to the combo box
    let mut api = YGOProDeckAPI::new(APIConfig::new("https://db.ygoprodeck.com/api/v7"));

    let cardsets = api.get_cardsets().unwrap();
    for set in cardsets.iter() {
        store.set(&store.append(), &[0], &[&set.set_name]);
    }

    // TODO: remove this, since it is for testing purposes
    let cards = api
        .get_cardinfo(&CardInfoRequest {
            name: None,
            cardset: Some(cardsets[0].set_name.clone()),
        })
        .unwrap();

    println!("{:?}", cards);
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
