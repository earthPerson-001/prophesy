use glib::Object;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use std::fs;
use std::io::prelude::*;
use std::path::Path;

use crate::utils::load_historic_data::load_data;

mod imp {

    use gtk::glib::{log_structured, LogLevel};

    use crate::utils::load_historic_data::ChargeState;

    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/prophesy/ui/prophesy_dashboard.ui")]
    pub struct Dashboard {}

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
            for (f, d) in battery_history_data.iter() {
                let mut file_basename = f
                    .basename()
                    .expect("Couldn't get file basename for battery report file.");
                file_basename.set_extension(".csv");
                
                let file_name = file_basename.file_name().expect("Couldn't get filename for battery history file");

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
