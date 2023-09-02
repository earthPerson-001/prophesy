use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::gio::Settings;
use std::cell::OnceCell;

// Object holding the state
#[derive(Default)]
pub struct ApplicationWindow {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "ProphesyApplicationWindow";
    type Type = super::ApplicationWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl WidgetImpl for ApplicationWindow {}

impl WindowImpl for ApplicationWindow {
    // Save window state right before the window will be closed
    fn close_request(&self) -> glib::Propagation {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");
        // Allow to invoke other event handlers
        glib::Propagation::Proceed
    }
}

impl ApplicationWindowImpl for ApplicationWindow {}

// Trait shared by all GObjects
impl ObjectImpl for ApplicationWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

