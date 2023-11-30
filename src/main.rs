
use gtk::glib;
use gtk::glib::{log_structured, LogLevel};
use gtk::prelude::*;
use gtk::gio;

use gtk::{gdk::Display, CssProvider, IconTheme};

mod application_window;
use application_window::ApplicationWindow;

mod utils;

mod application_pane;
mod left_pane;
mod right_pane;

mod dashboard;
mod history;
mod suggestion;
mod about;

// The application name
const APPLICATION_NAME: &str = "Prophesy";
// Unique identifier for the application
const APPLICATION_ID: &str = "com.prophesy";

fn main() -> glib::ExitCode {

    // Register and include resources
    // used to load resources (for ui files)
    gio::resources_register_include!("prophesy.gresource")
        .expect("Failed to register resources.");

    let application = gtk::Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    // The whole UI is built in this
    // and all other side-effects while application is running
    application.connect_startup(|_| load_css());
    application.connect_startup(|_| load_icons());

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

fn load_icons() {
    IconTheme::default().add_resource_path("/com/prophesy/icons");
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    let css_path = "/com/prophesy/styles/prophesy.css";
    provider.load_from_resource(css_path);

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(application: &gtk::Application) {

    let window = ApplicationWindow::new(application);

    window.set_title(Some(APPLICATION_NAME));

    window.present();
}
