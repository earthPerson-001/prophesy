use glib::subclass::Signal;
use glib::Properties;
use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, CompositeTemplate};
use once_cell::sync::Lazy;

use glib::Object;

use std::cell::Cell;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Properties, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_left_pane.ui")]
    #[properties(wrapper_type = super::LeftPane)]
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

        #[property(
            name = "left-pane-currently-active-tab",
            set,
            get,
            builder(Tabs::default())
        )]
        pub currently_active_tab: Cell<Tabs>,
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
    #[glib::derived_properties]
    impl ObjectImpl for LeftPane {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("tab-switched")
                    .param_types([Tabs::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }

        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();
            self.about_icon.set_from_icon_name(Some("about-icon"));
        }
    }

    #[gtk::template_callbacks]
    impl LeftPane {
        #[template_callback]
        fn on_navigation_button_clicked(&self, button: &gtk::ToggleButton) {
            let maybe_container = button.child();
            if let Some(container) = maybe_container {
                let maybe_label = container.last_child();

                if let Some(label) = maybe_label {
                    if label.is::<gtk::Label>() {
                        let maybe_label_widget = label.downcast::<gtk::Label>();

                        if let Ok(label_widget) = maybe_label_widget {
                            let currently_active_tab_name: glib::GString = self.currently_active_tab.get().into();
                            if currently_active_tab_name != label_widget.label() {
                                self.currently_active_tab
                                    .set(Tabs::from(Some(label_widget.label())));

                                self.obj().emit_by_name::<()>(
                                    "tab-switched",
                                    &[&Tabs::from(Some(label_widget.label()))],
                                );
                            }

                            return;
                        }
                    }
                }
            }

            dbg!("Cannot convert from {} to a LeftPane::Tabs, defaulting to default: ");
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

#[derive(Default, Debug, Copy, Clone, glib::Enum)]
#[enum_type(name = "Tabs")]
pub enum Tabs {
    #[default]
    Dashboard,
    History,
    Suggestion,
    About,
}

impl From<Tabs> for glib::GString {
    fn from(val: Tabs) -> Self {
        match val {
            Tabs::Dashboard => {glib::GString::from("Dashboard".to_string())},
            Tabs::History => {glib::GString::from("History".to_string())},
            Tabs::Suggestion => {glib::GString::from("Suggestion".to_string())},
            Tabs::About => {glib::GString::from("About".to_string())}
        }
    }
}

impl From<Option<glib::GString>> for Tabs {
    fn from(value: Option<glib::GString>) -> Self {
        match value {
            Some(g_string) => {
                if g_string == "Dashboard" {
                    Tabs::Dashboard
                } else if g_string == "History" {
                    Tabs::History
                } else if g_string == "Suggestion" {
                    Tabs::Suggestion
                } else if g_string == "About" {
                    Tabs::About
                } else {
                    dbg!("Conversion of 'string' to enum Tabs failed. Defaulting to default.");
                    Tabs::default()
                }
            }
            None => {
                dbg!("Conversion of 'string' to enum Tabs failed. Defaulting to default.");
                Tabs::default()
            }
        }
    }
}
