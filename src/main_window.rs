use crate::application_window::ApplicationWindow;
use gtk::{prelude::*, Box};
use gtk::{Button, Image, MenuButton, Stack, StackSidebar};
use gtk::{HeaderBar, Label};

use crate::utils::load_historic_data::load_data;

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
    let sections = [
        "Data Loading",
        "Charging and Discharging History",
        "Battery Health History",
        "Charging and Discharging Prediction",
        "Battery/Power Information",
        "Suggestions",
        "About",
    ];

    // adding the data loading section
    let data_loading_section_label = Label::new(Some("Loading the data"));
    let data_loading_section_title = sections[0];
    load_data();
    stack
        .add_named(
            &data_loading_section_label,
            Some(data_loading_section_title),
        )
        .set_title(data_loading_section_title);

    sections[1..].iter().for_each(|sec| {
        stack
            .add_named(&Label::new(Some(sec)), Some(sec))
            .set_title(sec);
    });

    app_window.set_child(Some(&main_container));
}
