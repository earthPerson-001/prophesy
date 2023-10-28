use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use glib::Object;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_about.ui")]
    pub struct About {

    }

    #[glib::object_subclass]
    impl ObjectSubclass for About {
        const NAME: &'static str = "ProphesyAbout";
        type Type = super::About;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for About {}

    impl BoxImpl for About {}

    // Trait shared by all GObjects
    impl ObjectImpl for About {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();
        }
    }
}

glib::wrapper! {
    pub struct About(ObjectSubclass<imp::About>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable;
}

impl Default for About {
    fn default() -> Self {
        Self::new()
    }
}

impl About {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
