extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate yugioh_card_entry;

use std::path::Path;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, ComboBox, IconView, ListStore};

use yugioh_card_entry::{APIConfig, CardInfoRequest, ImageCache, YGOProDeckAPI, API};

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
            cardset: Some(cardsets[3].set_name.clone()),
        })
        .unwrap();

    println!("{:?}", cards);

    // Set the cards
    let cards_store = ListStore::new(&[glib::Type::String, gdk_pixbuf::Pixbuf::static_type()]);
    let card_icon_view: IconView = builder.get_object("card_item_view").unwrap();
    card_icon_view.set_model(Some(&cards_store));

    let image_cache = ImageCache {
        cache_path: ".".to_string(),
    };

    let width = 200;
    let height = 200;
    for card in cards.iter() {
        let image_path = image_cache
            .get_image(&card.card_images[0].image_url)
            .unwrap();

        let image =
            gdk_pixbuf::Pixbuf::from_file_at_scale(Path::new(&image_path), width, height, true)
                .unwrap();

        cards_store.set(&cards_store.append(), &[0, 1], &[&card.name, &image])
    }

    println!("num cards: {}", cards.len());

    card_icon_view.set_text_column(0);
    card_icon_view.set_pixbuf_column(1);
    card_icon_view.set_item_width(100);

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
