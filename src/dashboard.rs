use glib::Object;
use gtk::glib::clone;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

use crate::utils::battery_historic_data::{load_data, DataValue};
use crate::utils::battery_realtime_data::BatteryInfo;

use gtk::glib::{log_structured, LogLevel};

use crate::utils::battery_historic_data::ChargeState;

mod imp {

    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_dashboard.ui")]
    pub struct Dashboard {
        #[template_child]
        pub page_title: TemplateChild<gtk::Box>,
        #[template_child]
        pub graph_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub labels_box: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Dashboard {
        const NAME: &'static str = "ProphesyDashboard";
        type Type = super::Dashboard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for Dashboard {}

    impl BoxImpl for Dashboard {}

    // Trait shared by all GObjects
    impl ObjectImpl for Dashboard {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();

            // loading the data
            let battery_history_data = load_data();
            let battery_status = Rc::new(RefCell::new(BatteryInfo::new()));

            // debug: saving for exporting
            save_data_as_csv(battery_history_data);

            // some buttons

            let refresh_button = gtk::Button::builder()
                .label("Refresh")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            let debug_print_battery_info_button = gtk::Button::builder()
                .label("Debug:print_battery_info")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            // labels

            let current_charge_label = gtk::Label::builder()
                .label(format!(
                    "{:?}",
                    battery_status.borrow().state_of_charge.abs()
                )) // todo: do it properly
                .margin_top(10)
                .margin_bottom(10)
                .margin_start(10)
                .margin_end(10)
                .build();

            // refresh on button clicked
            refresh_button.connect_clicked(clone!(@strong battery_status, @weak current_charge_label => move |_| {
                battery_status.borrow_mut().update_info();
                current_charge_label.set_label(format!("{:?}", battery_status.borrow().state_of_charge.abs()).as_str());
            }));

            // dump battery info
            debug_print_battery_info_button.connect_clicked(
                clone!(@weak battery_status => move |_| {
                    battery_status.borrow().print_info();
                }),
            );

            // add buttons
            self.page_title.append(&refresh_button);
            self.page_title.append(&debug_print_battery_info_button);

            // add graph

            // add labels
            let row_1 = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(10)
                .build();
            row_1.append(&gtk::Label::new(Some("Current Charge Level: ")));
            row_1.append(&current_charge_label);

            
            self.labels_box.append(&row_1);
        }
    }

    fn save_data_as_csv(
        battery_history_data: HashMap<gtk::gio::File, HashMap<gtk::glib::DateTime, DataValue>>,
    ) {
        for (f, d) in battery_history_data.iter() {
            let mut file_basename = f
                .basename()
                .expect("Couldn't get file basename for battery report file.");
            file_basename.set_extension(".csv");

            let file_name = file_basename
                .file_name()
                .expect("Couldn't get filename for battery history file");

            let battery_history_file_path = Path::new(file_name);

            let mut battery_history_file = fs::File::create(battery_history_file_path)
                .expect("Couldn't create battery_history_file");

            for (dt, dv) in d.iter() {
                battery_history_file
                    .write_fmt(format_args!(
                        "{},{},{}\n",
                        dt.to_unix(),
                        dv.value,
                        match dv.charge_state {
                            ChargeState::Charging => {
                                "Charging"
                            }
                            ChargeState::Discharging => {
                                "Discharging"
                            }
                            ChargeState::Unknown => {
                                "Unknown"
                            }
                        }
                    ))
                    .unwrap();
            }
            log_structured!("Prophesy",
            LogLevel::Debug,
            {
                "MESSAGE" => std::format!("Battery history saved to {:?}", battery_history_file_path.as_os_str())
            });
        }
    }
}

glib::wrapper! {
    pub struct Dashboard(ObjectSubclass<imp::Dashboard>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable;
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Dashboard {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
