use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use battery_data_analysis::{battery_plot_pdf, get_data_from_csv, CairoBackend};
use glib::Object;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(Debug, Default)]
    pub struct Graph {}

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
            let maybe_data = get_data_from_csv("E:\\prophesy\\batteryreport.csv");
            battery_plot_pdf(backend, maybe_data.unwrap()).unwrap();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Graph {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Load latest window state
            let _ = self.obj();
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
