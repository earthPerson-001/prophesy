use crate::application_window::ApplicationWindow;
use gtk::{prelude::*, Box};
use gtk::{Button, Image, MenuButton, Stack, StackSidebar};
use gtk::{HeaderBar, Label};

use crate::utils::load_historic_data::load_data;

use starship_battery as battery;

pub fn build_main_window(app_window: &ApplicationWindow) {
    // the title bar

    let head_bar = HeaderBar::new();
    head_bar.set_show_title_buttons(true);
    head_bar.pack_start(&Image::builder().icon_name("battery-full").build());
    head_bar.pack_start(&MenuButton::builder().icon_name("start-here").build());

    head_bar.pack_end(&Button::builder().icon_name("emblem-system").build());
    app_window.set_titlebar(Some(&head_bar));

    // the application body

    let main_container = Box::new(gtk::Orientation::Horizontal, 10);

    let side_bar = StackSidebar::new();
    main_container.append(&side_bar);

    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideUpDown);
    stack.set_hexpand(true);

    // using the same stack for both
    side_bar.set_stack(&stack);

    main_container.append(&stack);

    // Adding various sections
    let sections = ["Dashboard", "History", "Suggestions", "About"];

    // adding the dashboard section
    let dashboard_section_title = sections[0];
    let dashboard_container = Box::new(gtk::Orientation::Vertical, 10);

    // using starship-battery to extract battery information
    let manager = battery::Manager::new();

    if let Ok(mgr) = manager {
        for (idx, maybe_battery) in mgr.batteries().unwrap().enumerate() {
            let battery = maybe_battery.unwrap();
            dashboard_container.append(&Label::new(Some(format!("Battery #{}:", idx).as_str())));
            dashboard_container.append(&Label::new(Some(
                format!("Vendor: {:?}", battery.vendor()).as_str(),
            )));
            dashboard_container.append(&Label::new(Some(
                format!("Model: {:?}", battery.model()).as_str(),
            )));
            dashboard_container.append(&Label::new(Some(
                format!("State: {:?}", battery.state()).as_str(),
            )));
            if let Some(full_charge_time) = battery.time_to_full() {
                dashboard_container.append(&Label::new(Some(
                    format!("Time to full charge: {:?}", full_charge_time).as_str(),
                )));
            }
            if let Some(time_to_emtpy) = battery.time_to_empty() {
                dashboard_container.append(&Label::new(Some(
                    format!("Time to emtpy: {:?}", time_to_emtpy).as_str(),
                )));
            }
        }
    }

    // history section
    let history_section_title = sections[1];
    let history_container = Box::new(gtk::Orientation::Vertical, 10);

    // getting the history
    let files_and_data = load_data();

    for (_file, entries) in files_and_data {
        let file_path = _file.path().unwrap();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        print!("{}", file_name);

        for (dt, entry) in entries {
            if dt != entry.date_time {
                eprintln!(
                    "key: {:?} and entry: {:?} aren't equal",
                    dt, entry.date_time
                );
            }
            dbg!(
                "time: {}-{}-{} {}hr {}m {}s, value: {}, state: {:?}",
                entry.date_time.year(),
                entry.date_time.month(),
                entry.date_time.day_of_month(),
                entry.date_time.hour(),
                entry.date_time.minute(),
                entry.date_time.seconds(),
                entry.value,
                entry.charge_state
            );
        }
    }

    stack
        .add_named(&dashboard_container, Some(dashboard_section_title))
        .set_title(dashboard_section_title);

    stack
        .add_named(&history_container, Some(history_section_title))
        .set_title(history_section_title);

    sections[2..].iter().for_each(|sec| {
        stack
            .add_named(&Label::new(Some(sec)), Some(sec))
            .set_title(sec);
    });

    app_window.set_child(Some(&main_container));
}
