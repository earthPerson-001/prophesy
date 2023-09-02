use gtk::glib;
use gtk::glib::{log_structured, LogLevel};
use gtk::prelude::*;
use gtk::gio;

mod main_window;
use main_window::build_main_window;

mod load_historic_data;

pub mod application_window;
use application_window::ApplicationWindow;

// The application name
const APPLICATION_NAME: &str = "Prophesy";
// Unique identifier for the application
const APPLICATION_ID: &str = "com.prophesy";

fn main() -> glib::ExitCode {

    // Register and include resources
    // used to load resources (for ui files)
    // TODO build ui through this ( it isn't used for now)
    gio::resources_register_include!("prophesy.gresource")
        .expect("Failed to register resources.");

    let application = gtk::Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    // The whole UI is built in this
    // and all other side-effects while application is running
    application.connect_activate(build_ui);

    // Logging some values on activate
    application.connect_activate(|_| {
        log_structured!(
            "Prophesy",
            LogLevel::Debug,
            {
                "LOGGED_AT" => "Beginning";
                "MESSAGE" => "com.prophesy started successfully.";
            }
        )
    });

    // Logging some values on shutdown
    application.connect_shutdown(|_| {
        log_structured!(
            "Prophesy",
            LogLevel::Debug,
            {
                "LOGGED_AT" => "End";
                "MESSAGE" => "com.prophesy terminated.";
            }
        )
    });

    application.run()
}

fn build_ui(application: &gtk::Application) {

    let window = ApplicationWindow::new(application);

    window.set_title(Some(APPLICATION_NAME));

    build_main_window(&window);

    window.present();
}
