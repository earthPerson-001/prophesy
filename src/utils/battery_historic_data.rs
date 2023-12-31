use gtk::gio::File;
use gtk::glib::log_structured;
use gtk::glib::DateTime;
use gtk::glib::LogLevel;
use std::collections::HashMap;
use std::path::Path;
use std::process::{Child, Command};

#[derive(Debug)]
pub enum ChargeState {
    Charging,
    Discharging,
    Unknown,
}

#[derive(Debug)]
pub struct DataValue {
    pub date_time: DateTime,
    pub value: f32,
    pub charge_state: ChargeState,
}

#[cfg(target_os = "linux")]
pub fn load_data() -> HashMap<File, HashMap<DateTime, DataValue>> {
    use gtk::{gio::Cancellable, glib::log_structured};

    // considering all the paths are valid UTF-8 strings

    let data_dir = File::for_path(Path::new(r"/var/lib/upower/"));
    let pattern_for_data_files =
        String::from(data_dir.path().unwrap().to_str().unwrap()) + "/*.dat";
    let data_files = glob(&pattern_for_data_files).expect("Failed to read glob pattern");

    let mut files: Vec<File> = Vec::new();

    let mut files_and_data: HashMap<File, HashMap<DateTime, DataValue>> = HashMap::new();

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
    for file in files {
        dbg!(
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

        files_and_data.insert(file, dat_as_struct);
    }

    return files_and_data;
}

#[cfg(target_os = "windows")]
/**
 # parameters
 file_path: the path where the generated xml will be stored

 number_of_days : max 14 days

 # Bug
 New batteryreport is create on each call irrespective of previous reports. 
 So, this should be call infrequently(ideally only once).
*/
fn create_battery_report(file_path: &Path, number_of_days: u8) -> Child {
    Command::new("powercfg")
        .args([
            "/batteryreport",
            "/output",
            file_path.to_str().unwrap(),
            "/xml",
            "/duration",
            number_of_days.to_string().as_str()
        ])
        .spawn()
        .expect("failed to execute child")
}

#[cfg(target_os = "windows")]
pub fn load_data() -> HashMap<File, HashMap<DateTime, DataValue>> {
    let mut files_and_data: HashMap<File, HashMap<DateTime, DataValue>> = HashMap::new();

    use std::{fs, io::Read};

    use gtk::glib::TimeZone;

    let battery_report_filepath = Path::new("./batteryreport.xml");
    let mut battery_report_subprocess = create_battery_report(battery_report_filepath, 14);
    battery_report_subprocess
        .wait()
        .expect("Couldn't complete the creation of batteryreport.");

    let battery_report_file = fs::File::open(battery_report_filepath);

    if let Ok(mut file) = battery_report_file {
        let mut battery_report_string = String::new();
        file.read_to_string(&mut battery_report_string)
            .expect("Couldn't read batteryreport file.");

        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        match roxmltree::Document::parse_with_options(&battery_report_string, opt) {
            Ok(doc) => {
                let mut descendants_of_root = doc.descendants();
                let system_information =
                    descendants_of_root.find(|n| n.has_tag_name("SystemInformation"));

                log_structured!("Prophesy",
                LogLevel::Debug,
                {
                    "MESSAGE" => std::format!("--------Sytem Information__________")
                });

                if let Some(info) = system_information {
                    let childrens = info.children().filter(|c| c.tag_name().name() != "");

                    for child in childrens {
                        if let Some(text) = child.text() {
                            log_structured!("Prophesy",
                            LogLevel::Debug,
                            {
                                "MESSAGE" => std::format!("{} : {}", child.tag_name().name(), text)
                            });
                        }
                    }
                }

                log_structured!("Prophesy",
                LogLevel::Debug,
                {
                    "MESSAGE" => std::format!("--------Battery Information__________")
                });

                let batteries_node = descendants_of_root.find(|n| n.has_tag_name("Batteries"));

                if let Some(node) = batteries_node {
                    let batteries = node.children();

                    for battery in batteries {
                        for value in battery.children().filter(|c| c.tag_name().name() != "") {
                            if let Some(text) = value.text() {
                                log_structured!("Prophesy",
                                LogLevel::Debug,
                                {
                                    "MESSAGE" => std::format!("{} : {}", value.tag_name().name(), text)
                                });
                            }
                        }
                    }
                }

                let mut dat_as_struct = HashMap::<DateTime, DataValue>::new();

                let recent_usages_node =
                    descendants_of_root.find(|n| n.has_tag_name("RecentUsage"));

                if let Some(node) = recent_usages_node {
                    let usage_entries = node
                        .children()
                        .filter(|c| c.text() != Some("") && c.text() != Some("\n    "));

                    for usage in usage_entries {

                        let time_stamp_option = usage.attribute("Timestamp");

                        if time_stamp_option.is_none() {continue;}

                        let on_ac_option = usage.attribute("Ac");
                        let charge_capacity_option = usage.attribute("ChargeCapacity");

                        let xml_deserializer = move || -> Option<DataValue> {
                            // powercfg stores time in iso8601 format
                            // using utc offset
                            let local_time_stamp = DateTime::from_iso8601(
                                time_stamp_option.unwrap(),
                                Some(&TimeZone::new(Some("Z"))),
                            )
                            .unwrap();
                            let on_ac = on_ac_option?;
                            let charge_capacity = charge_capacity_option?.parse::<f32>().unwrap();

                            let charge_state = {
                                if on_ac == "1" {
                                    ChargeState::Charging
                                } else {
                                    ChargeState::Discharging
                                }
                            };

                            Some(DataValue {
                                date_time: local_time_stamp,
                                value: charge_capacity,
                                charge_state,
                            })
                        };

                        if let Some(dv) = xml_deserializer() {
                            dat_as_struct.insert(dv.date_time.clone(), dv);
                        }
                    }
                }

                files_and_data.insert(
                    gtk::gio::File::for_path(battery_report_filepath),
                    dat_as_struct,
                );
            }
            Err(e) => log_structured!("Prophesy",
            LogLevel::Debug,
            {
                "MESSAGE" => std::format!("Couldn't parse {} because of error {:?}", battery_report_filepath.to_str().unwrap() ,e)
            }),
        }
    }

    files_and_data
}

#[cfg(target_os = "macos")]
pub fn load_data() -> HashMap<File, HashMap<DateTime, DataValue>> {
    let mut files_and_data: HashMap<File, HashMap<DateTime, DataValue>> = HashMap::new();

    return files_and_data;
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod windows_tests {
    use std::{fs::File, io::Read, path::Path};

    use super::create_battery_report;

    static BATTTERY_REPORT_XML: &str = include_str!("batteryreport.xml");

    #[test]
    fn load_xml_test() {
        fn test_str(text: &str) {
            let opt = roxmltree::ParsingOptions {
                allow_dtd: true,
                ..roxmltree::ParsingOptions::default()
            };
            match roxmltree::Document::parse_with_options(text, opt) {
                Ok(_) => (),
                Err(e) => panic!("Couldn't load the provided xml. Error {e}"),
            }
        }

        let text = BATTTERY_REPORT_XML;
        test_str(text);

        let battery_report_filepath = Path::new("./batteryreport.xml");
        let mut battery_report_subprocess = create_battery_report(battery_report_filepath, 3);
        battery_report_subprocess
            .wait()
            .expect("Couldn't complete the creation of batteryreport.");

        let battery_report_file = File::open(battery_report_filepath);

        if let Ok(mut file) = battery_report_file {
            let mut battery_report_string = String::new();
            file.read_to_string(&mut battery_report_string)
                .expect("Couldn't read batteryreport file.");

            test_str(&battery_report_string);
        }
    }

    #[test]
    fn parse_xml_test() {}
}
