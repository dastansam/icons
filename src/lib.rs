use gtk::{gdk, gio, glib};
use gvdb::gresource::{GResourceBuilder, GResourceFileData, PreprocessOptions};
use std::{ops::Deref, sync::OnceLock};

pub mod icon_modules;

const GENERAL_PREFIX: &str = "/org/gtkrs/icons/scalable/actions/";

pub struct LazyIcon {
    name: &'static str,
    data: &'static [u8],
    initialized: OnceLock<()>,
}

impl LazyIcon {
    pub const fn new(name: &'static str, data: &'static [u8]) -> Self {
        LazyIcon {
            name,
            data,
            initialized: OnceLock::new(),
        }
    }

    fn init(&self) {
        self.initialized.get_or_init(|| {
            let file_data = GResourceFileData::new(
                [GENERAL_PREFIX, self.name, ".svg"].into_iter().collect(),
                std::borrow::Cow::Borrowed(self.data),
                None,
                true,
                &PreprocessOptions::xml_stripblanks(),
            )
            .expect("Failed to create file data");

            let resources = vec![file_data];
            let data = GResourceBuilder::from_file_data(resources)
                .build()
                .expect("Failed to build resource bundle");

            let resource = gio::Resource::from_data(&glib::Bytes::from(&data))
                .expect("Failed to create resource");
            gio::resources_register(&resource);

            println!("Registered icon: {}", self.name);
        });
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl Deref for LazyIcon {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.init();
        self.name
    }
}

pub fn initialize_icons() {
    gtk::init().unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/org/gtkrs/icons/");
    theme.add_resource_path("/org/gtkrs/icons/scalable/actions/");
}
