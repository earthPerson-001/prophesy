
use glob::glob;
use gtk::gio::prelude::*;
use gtk::gio::File;
use gtk::glib::DateTime;
use gtk::glib::LogLevel;
use std::path::Path;

#[derive(Debug)]
enum ChargeState {
    Charging,
    Discharging,
    Unknown,
}

#[derive(Debug)]
struct DataValue {
    date_time: DateTime,
    value: f32,
    charge_state: ChargeState,
}

#[cfg(target_os = "linux")]
pub fn load_data() {
    // considering all the paths are valid UTF-8 strings

    use std::collections::HashMap;

    use gtk::{gio::Cancellable, glib::log_structured};

    let data_dir = File::for_path(Path::new(r"/var/lib/upower/"));
    let pattern_for_data_files =
        String::from(data_dir.path().unwrap().to_str().unwrap()) + "/*.dat";
    let data_files = glob(&pattern_for_data_files).expect("Failed to read glob pattern");

    let mut files: Vec<File> = Vec::new();

    // keeping track of different files
    for file in data_files {
        match file {
            Ok(path) => {
                // consider only files which don't have generic in the name
                if path.is_file() && !String::from(path.to_str().unwrap()).contains("generic") {
                    files.push(File::for_path(path))
                }
            }
            Err(e) => log_structured!("Prophesy",
            LogLevel::Debug,
            {
                "MESSAGE" => e.to_string();
            }),
        }
    }

    // file read cancellable
    let cancellable = Cancellable::new();

    // reading the data of the files
    files.iter().for_each(|file| {
        println!(
            "\nReading from file {}",
            file.path().unwrap().to_str().unwrap()
        );
        let mut file_buffer = [0; 1000]; // the compiler won't warn but a reference to a mutable buffer is required
        let input_stream = file.read(Some(&cancellable)).unwrap();
        let _ = input_stream.read_all(&mut file_buffer, Some(&cancellable));

        // as string
        let buffer_as_string = std::str::from_utf8(&file_buffer).unwrap();
        let lines = buffer_as_string.split('\n');

        // hashmap containing each value in the file
        let mut dat_as_struct: HashMap<DateTime, DataValue> = HashMap::new();

        for line in lines {
            let vals: Vec<&str> = line.split('\t').collect();

            // continue if there are less than three columns
            if vals.len() < 3 {
                continue;
            }  

            // first column is time
            let date_time = DateTime::from_unix_utc(vals[0].parse::<i64>().unwrap());
            let val = vals[1].parse::<f32>().unwrap();
            let charge_state = {
                match vals[2] {
                    "charging" => ChargeState::Charging,
                    "discharging" => ChargeState::Discharging,
                    _ => ChargeState::Unknown,
                }
            };
            
            if let Ok(dt) = date_time {
                println!("time: {}-{}-{} {}hr {}m {}s, value: {}, state: {:?}", dt.year(), dt.month(), dt.day_of_month(), dt.hour(), dt.minute(), dt.seconds(), val, charge_state);
                dat_as_struct.insert(
                    dt.clone(),
                    DataValue {
                        date_time: dt,
                        value: val,
                        charge_state,
                    },
                );
            }
        }
    });
}

#[cfg(target_os = "windows")]
pub fn load_data() {}

#[cfg(target_os = "macos")]
pub fn load_data() {}
