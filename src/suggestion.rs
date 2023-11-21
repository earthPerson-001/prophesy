use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use glib::Object;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_suggestion.ui")]
    pub struct Suggestion {

    }

    #[glib::object_subclass]
    impl ObjectSubclass for Suggestion {
        const NAME: &'static str = "ProphesySuggestion";
        type Type = super::Suggestion;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for Suggestion {}

    impl BoxImpl for Suggestion {}

    // Trait shared by all GObjects
    impl ObjectImpl for Suggestion {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();
        }
    }
}

glib::wrapper! {
    pub struct Suggestion(ObjectSubclass<imp::Suggestion>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable;
}

impl Default for Suggestion {
    fn default() -> Self {
        Self::new()
    }
}

impl Suggestion {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
