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

use glib::Properties;

mod imp {

    use std::{cell::Cell, cell::RefCell, collections::HashMap};
    use super::*;

    // Object holding the state
    #[derive(CompositeTemplate, Default, Properties)]
    #[properties(wrapper_type = super::Dashboard)]
    #[template(resource = "/com/prophesy/ui/prophesy_dashboard.ui")]
    pub struct Dashboard {
        #[template_child]
        pub page_title: TemplateChild<gtk::Box>,
        #[template_child]
        pub graph_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub current_percentage_label_value: TemplateChild<gtk::Label>,
        #[template_child]
        pub battery_health_label_value: TemplateChild<gtk::Label>,
        #[template_child]
        pub remaining_screen_on_time_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub remaining_screen_on_time_label_value: TemplateChild<gtk::Label>,
        #[template_child]
        pub estimated_time_on_full_charge_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub estimated_time_on_full_chage_label_value: TemplateChild<gtk::Label>,
        #[template_child]
        pub current_drain_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub current_drain_label_value: TemplateChild<gtk::Label>,


        #[property(get, set, default_value = 0.0)]
        current_percentage: Cell<f32>,
        #[property(get, set, default_value = 0.0)]
        battery_health: Cell<f32>,
        #[property(get, set, default_value = 0.0)]
        current_drain: Cell<f32>,
        #[property(get, set, default_value = 0.0)]
        remaining_time: Cell<f32>,
        #[property(get, set, default_value = 0.0)]
        estimated_time_on_full_charge: Cell<f32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Dashboard {
        const NAME: &'static str = "ProphesyDashboard";
        type Type = super::Dashboard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            crate::graph::Graph::ensure_type();
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
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            Self::derived_set_property(self, id, value, pspec);
            self.obj().queue_draw();
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            Self::derived_property(self, id, pspec)
        }

        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let obj = self.obj();

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

            // refresh on button clicked
            refresh_button.connect_clicked(clone!(@strong battery_status, @weak self as this => move |_| {
                battery_status.borrow_mut().update_info();

                // debug
                battery_status.borrow().print_info();


                this.obj().set_current_percentage((battery_status.borrow().state_of_charge.abs()*uom::si::f32::Ratio::new::<uom::si::ratio::part_per_hundred>(10000.0)).value);                
                this.obj().set_battery_health((battery_status.borrow().state_of_health.abs()*uom::si::f32::Ratio::new::<uom::si::ratio::part_per_hundred>(10000.0)).value);
                this.obj().set_current_drain((battery_status.borrow().energy_rate).value);
                let remaining_time = match battery_status.borrow().state {
                            starship_battery::State::Unknown => {
                                let state_unknown = "Charge State Unknown";
                                this.obj().imp().remaining_screen_on_time_label.set_label(state_unknown);
                                this.obj().imp().current_drain_label.set_label("Current Drain");
                                None
                            }
                            starship_battery::State::Charging => {
                                let charging_info = "Remaining Time For Full Charge";
                                this.obj().imp().remaining_screen_on_time_label.set_label(charging_info);
                                this.obj().imp().current_drain_label.set_label("Current Charge Rate");
                                battery_status.borrow().time_to_full
                            },
                            starship_battery::State::Discharging => {
                                let discharging_info = "Remaining Screen On Time";
                                this.obj().imp().remaining_screen_on_time_label.set_label(discharging_info);
                                battery_status.borrow().time_to_empty
                            },
                            starship_battery::State::Empty => {
                                let battery_empty = "Battery Empty";
                                this.obj().imp().remaining_screen_on_time_label.set_label(battery_empty);
                                None
                            },
                            starship_battery::State::Full => {
                                let battery_full = "Battery Full";
                                this.obj().imp().remaining_screen_on_time_label.set_label(battery_full);
                                None
                            },
                        }; 
                if let Some(rt) = remaining_time {
                    this.obj().set_remaining_time(rt.value);
                } 
            }));
        
            // binding labels
            obj.bind_property(
                "current_percentage",
                &obj.imp().current_percentage_label_value.get(),
                "label",
            )
            .transform_to(|_, percentage: f32| {
                // casting to u8 from float as we don't need that precision for battery percentage
                Some(format!("{:2}%", percentage.floor() as u8))
            })
            .sync_create()
            .build();

            obj.bind_property(
                "battery_health",
                &obj.imp().battery_health_label_value.get(),
                "label",
            )
            .transform_to(|_, percentage: f32| {
                Some(format!("{:.2}%", percentage))
            })
            .sync_create()
            .build();

            obj.bind_property(
                "remaining_time",
                &obj.imp().remaining_screen_on_time_label_value.get(),
                "label",
            )
            .transform_to(|_, time: f32| {
                Some(format!("{} Seconds", time))
            })
            .sync_create()
            .build();

            obj.bind_property(
                "current_drain",
                &obj.imp().current_drain_label_value.get(),
                "label",
            )
            .transform_to(|_, power: f32| {
                Some(format!("{} Watts", power))
            })
            .sync_create()
            .build();

            // add buttons
            self.page_title.append(&refresh_button);


            // refreshing initially
            // todo: change this behaviour to get the correct data initially
            refresh_button.emit_clicked();
        }
    }

    fn save_data_as_csv(
        battery_history_data: HashMap<gtk::gio::File, HashMap<gtk::glib::DateTime, DataValue>>,
    ) {
        for (f, d) in battery_history_data.iter() {
            let mut file_basename = f
                .basename()
                .expect("Couldn't get file basename for battery report file.");
            file_basename.set_extension("csv");

            let file_name = file_basename
                .file_name()
                .expect("Couldn't get filename for battery history file");

            let battery_history_file_path = Path::new(file_name);

            let mut battery_history_file = fs::File::create(battery_history_file_path)
                .expect("Couldn't create battery_history_file");

            // the csv header
            battery_history_file
                .write_fmt(format_args!("date_time,capacity,state\n"))
                .unwrap();

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
