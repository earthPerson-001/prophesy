use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use glib::Object;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/../ui/prophesy_left_pane.ui")]
    pub struct LeftPane {
        #[template_child]
        pub dashboard_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub history_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub suggestion_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub about_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub battery_summary: TemplateChild<gtk::Box>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LeftPane {
        const NAME: &'static str = "ProphesyLeftPane";
        type Type = super::LeftPane;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for LeftPane {}

    impl BoxImpl for LeftPane{}

    // Trait shared by all GObjects
    impl ObjectImpl for LeftPane {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let obj = self.obj();

        }

    }
}

glib::wrapper! {
    pub struct LeftPane(ObjectSubclass<imp::LeftPane>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable;
}

impl Default for LeftPane {
    fn default() -> Self {
        Self::new()
    }
}

impl LeftPane {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
