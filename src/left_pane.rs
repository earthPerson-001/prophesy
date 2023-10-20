use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, CompositeTemplate};

use glib::Object;

mod imp {
    use gtk::IconTheme;

    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_left_pane.ui")]
    pub struct LeftPane {
        #[template_child]
        pub dashboard_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub dashboard_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub history_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub history_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub suggestion_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub suggestion_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub about_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub about_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub battery_summary: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LeftPane {
        const NAME: &'static str = "ProphesyLeftPane";
        type Type = super::LeftPane;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for LeftPane {}

    impl BoxImpl for LeftPane {}

    // Trait shared by all GObjects
    impl ObjectImpl for LeftPane {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();

            let theme = IconTheme::default();
            theme.add_resource_path("/com/prophesy/icons");
            let about_icon = theme.lookup_icon(
                "about-icon",
                &["help-about"],
                48,
                1,
                gtk::TextDirection::Ltr,
                gtk::IconLookupFlags::empty(),
            );
            // setting the imported icons
            self.about_icon.set_from_paintable(Some(&about_icon));
            // self.about_icon.set_from_icon_name(Some("about-icon"));
            // self.about_icon.set_from_resource(Some("/com/prophesy/icons/scalable/actions/about-icon.svg"));
        }
    }

    #[gtk::template_callbacks]
    impl LeftPane {
        #[template_callback]
        fn on_navigation_button_clicked(button: &gtk::ToggleButton) {
            // Set the label to "Hello World!" after the button has been clicked on
            dbg!("Pressed button {}", button);
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
