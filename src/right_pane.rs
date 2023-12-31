use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use glib::Object;

mod imp {
    use glib::Properties;
    use std::cell::Cell;

    use crate::{left_pane::Tabs, dashboard::Dashboard, history::History, suggestion::Suggestion, about::About};

    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default, Properties)]
    #[template(resource = "/com/prophesy/ui/prophesy_right_pane.ui")]
    #[properties(wrapper_type = super::RightPane)]
    pub struct RightPane {
        #[template_child]
        pub pages: TemplateChild<gtk::Notebook>,
        #[property(
            name = "right-pane-currently-active-tab",
            set,
            get,
            builder(Tabs::default())
        )]
        pub currently_active_tab: Cell<Tabs>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RightPane {
        const NAME: &'static str = "ProphesyRightPane";
        type Type = super::RightPane;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            // Register the types which aren't used as template childs
            Dashboard::ensure_type();
            History::ensure_type();
            Suggestion::ensure_type();
            About::ensure_type();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for RightPane {}

    impl BoxImpl for RightPane {}

    // Trait shared by all GObjects
    #[glib::derived_properties]
    impl ObjectImpl for RightPane {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();
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
