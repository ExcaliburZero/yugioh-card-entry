extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate home;
extern crate yugioh_card_entry;

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use futures::{channel::mpsc, StreamExt};

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, ComboBox, IconView, ListStore};

use yugioh_card_entry::{APIConfig, CardInfoRequest, ImageCache, YGOProDeckAPI, API};

use std::env::args;

type LockedAPI = Arc<Mutex<YGOProDeckAPI>>;
type LockedImageCache = Arc<Mutex<ImageCache>>;

fn build_ui(application: &gtk::Application) {
    // Setup image cache
    let mut cache_path = home::home_dir().unwrap();
    cache_path.push(".yugioh_card_entry");
    cache_path.push("images");

    let image_cache = Arc::new(Mutex::new(ImageCache::new(cache_path.to_str().unwrap())));

    // Load the UI elements from a Glade file
    let glade_src = include_str!("../resources/main.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder
        .get_object("main_window")
        .expect("Couldn't get main_window");
    window.set_application(Some(application));

    // Setup the card set combo box
    let store = ListStore::new(&[glib::Type::String]);

    let combo_box: ComboBox = builder.get_object("set_combo_box").unwrap();
    combo_box.set_model(Some(&store));
    combo_box.set_active(Some(0));

    // Get the card sets and add them to the combo box
    let api = Arc::new(Mutex::new(YGOProDeckAPI::new(APIConfig::new(
        "https://db.ygoprodeck.com/api/v7",
    ))));

    let cardsets = api.lock().unwrap().get_cardsets().unwrap();
    for set in cardsets.iter() {
        store.set(&store.append(), &[0], &[&set.set_name]);
    }

    // Set the visual settings for the card viewer
    let card_icon_view: IconView = builder.get_object("card_item_view").unwrap();

    card_icon_view.set_text_column(0);
    card_icon_view.set_pixbuf_column(1);
    card_icon_view.set_item_width(100);

    // Create a data model for the card viewer
    let cards_store = ListStore::new(&[glib::Type::String, gdk_pixbuf::Pixbuf::static_type()]);
    let card_icon_view: IconView = builder.get_object("card_item_view").unwrap();
    card_icon_view.set_model(Some(&cards_store));

    // Have card viewer update when user chooses a card set
    combo_box.connect_changed(move |cb| {
        let builder = builder.clone();

        let (sender, mut receiver) = mpsc::channel(1000);
        card_set_combobox_handler(cb, sender, api.clone(), image_cache.clone(), &builder);

        let main_context = glib::MainContext::default();
        let future = async move {
            while let Some(item) = receiver.next().await {
                let card_icon_view: IconView = builder.get_object("card_item_view").unwrap();
                let cards_store = card_icon_view
                    .get_model()
                    .unwrap()
                    .downcast::<ListStore>()
                    .unwrap();

                let (name, image_path) = item;

                let width = 200;
                let height = 200;
                println!("Attempting to open file: {}", image_path);
                let image = gdk_pixbuf::Pixbuf::from_file_at_scale(
                    Path::new(&image_path),
                    width,
                    height,
                    true,
                )
                .unwrap();

                cards_store.set(&cards_store.append(), &[0, 1], &[&name, &image]);
            }
        };
        main_context.spawn_local(future);
    });

    window.show_all();
}

fn combobox_get_active_value(combo_box: &ComboBox) -> String {
    let model = combo_box
        .get_model()
        .unwrap()
        .downcast::<ListStore>()
        .unwrap();

    let index = combo_box.get_active().unwrap();

    model
        .get_value(&model.iter_nth_child(None, index as i32).unwrap(), 0)
        .get()
        .unwrap()
        .unwrap()
}

fn card_set_combobox_handler(
    cb: &ComboBox,
    mut sender: mpsc::Sender<(String, String)>,
    api: LockedAPI,
    image_cache: LockedImageCache,
    builder: &gtk::Builder,
) {
    let cardset = combobox_get_active_value(cb);
    let cards = api
        .lock()
        .unwrap()
        .get_cardinfo(&CardInfoRequest {
            name: None,
            cardset: Some(cardset),
        })
        .unwrap();

    println!("{:?}", cards);

    // Set the cards
    let card_icon_view: IconView = builder.get_object("card_item_view").unwrap();
    let cards_store = card_icon_view
        .get_model()
        .unwrap()
        .downcast::<ListStore>()
        .unwrap();

    cards_store.clear();

    thread::spawn(move || {
        for card in cards.iter() {
            let image_path = image_cache
                .lock()
                .unwrap()
                .get_image(&card.card_images[0].image_url)
                .unwrap();

            println!("Got image: {}", image_path);

            sender.try_send((card.name.clone(), image_path)).unwrap();
        }

        println!("num cards: {}", cards.len());
    });
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
