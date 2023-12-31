use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use glib::Object;

mod imp {
    use gtk::glib::closure_local;

    use crate::{
        left_pane::{LeftPane, Tabs},
        right_pane::RightPane,
    };

    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_application_pane.ui")]
    pub struct ApplicationPane {
        #[template_child]
        pub left_child: TemplateChild<LeftPane>,
        #[template_child]
        pub right_child: TemplateChild<RightPane>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationPane {
        const NAME: &'static str = "ProphesyApplicationPane";
        type Type = super::ApplicationPane;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for ApplicationPane {}

    impl BoxImpl for ApplicationPane {}

    // Trait shared by all GObjects
    impl ObjectImpl for ApplicationPane {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let obj = self.obj();

            // binding buttons on left pane to the tabs on the right pane
            self.left_child.bind_property(
                "left-pane-currently-active-tab",
                &obj.imp().right_child.get(),
                "right-pane-currently-active-tab",
            ).sync_create().build();

            self.left_child.connect_closure(
                "tab-switched",
                false,
                closure_local!(@watch obj => move |_left_pane: LeftPane, switched_to: Tabs| {
                    obj.imp().right_child.get().imp().pages.set_current_page(Some(switched_to as u32));
                }),
            );
        }
    }
}

glib::wrapper! {
    pub struct ApplicationPane(ObjectSubclass<imp::ApplicationPane>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable;
}

impl Default for ApplicationPane {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationPane {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
