use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use glib::Object;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_history.ui")]
    pub struct History {

    }

    #[glib::object_subclass]
    impl ObjectSubclass for History {
        const NAME: &'static str = "ProphesyHistory";
        type Type = super::History;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for History {}

    impl BoxImpl for History {}

    // Trait shared by all GObjects
    impl ObjectImpl for History {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();
        }
    }
}

glib::wrapper! {
    pub struct History(ObjectSubclass<imp::History>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable;
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
