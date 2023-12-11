use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use battery_data_analysis::{battery_plot_pdf, display_error, get_data_from_csv, CairoBackend};
use glib::Object;

mod imp {
    use std::cell::Cell;

    use super::*;

    // Object holding the state
    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Graph)]
    pub struct Graph {
        #[property(get, set, minimum = 1, maximum = 14, default_value = 1)]
        start_day: Cell<i64>,
        #[property(get, set, minimum = 0, maximum = 13, default_value = 0)]
        end_day: Cell<i64>,
        #[property(get, set, default_value = false)]
        show_data_points: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Graph {
        const NAME: &'static str = "ProphesyGraph";
        type Type = super::Graph;
        type ParentType = gtk::Widget;
    }

    impl WidgetImpl for Graph {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let width = self.obj().width() as u32;
            let height = self.obj().height() as u32;
            if width == 0 || height == 0 {
                return;
            }

            let bounds = gtk::graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
            let cr = snapshot.append_cairo(&bounds);
            let backend = CairoBackend::new(&cr, (width, height)).unwrap();

            if self.end_day.get() >= self.start_day.get() {
                let error_msg = String::from("The range for from(days before) and to(days before) doesn't match. \n Error: 'to' must be less than 'from'");
                let mut max_line_len = 0;
                error_msg
                    .lines()
                    .for_each(|line| max_line_len = max_line_len.max(line.len()));
                display_error(
                    backend,
                    error_msg.as_str(),
                    (
                        (width - max_line_len as u32) as i32 / 2,
                        (height / 2) as i32,
                    ),
                );
            } else {
                let maybe_data = get_data_from_csv("E:\\prophesy\\batteryreport.csv");
                battery_plot_pdf(
                    backend,
                    maybe_data.unwrap(),
                    Some(self.start_day.get()),
                    Some(self.end_day.get()),
                    self.show_data_points.get(),
                )
                .unwrap();
            }
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Graph {
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
    }
}

glib::wrapper! {
    pub struct Graph(ObjectSubclass<imp::Graph>)
        @extends gtk::Widget;
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        // Create new window
        Object::builder().build()
    }
}
