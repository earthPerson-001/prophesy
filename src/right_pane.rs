use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use glib::Object;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_right_pane.ui")]
    pub struct RightPane {

    }

    #[glib::object_subclass]
    impl ObjectSubclass for RightPane {
        const NAME: &'static str = "ProphesyRightPane";
        type Type = super::RightPane;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for RightPane {}

    impl BoxImpl for RightPane{}

    // Trait shared by all GObjects
    impl ObjectImpl for RightPane {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let obj = self.obj();

        }

    }
}

glib::wrapper! {
    pub struct RightPane(ObjectSubclass<imp::RightPane>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable;
}

impl Default for RightPane {
    fn default() -> Self {
        Self::new()
    }
}

impl RightPane {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
