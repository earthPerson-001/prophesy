use battery_data_analysis::BatteryHistoryRecord;
use chrono::Utc;
use std::collections::HashMap;
use std::error::Error;

use battery_data_analysis::get_data_from_csv;

pub fn get_predicted_data(
) -> Result<HashMap<chrono::DateTime<Utc>, BatteryHistoryRecord>, Box<dyn Error>> {
    let mut predicted_pairs: HashMap<chrono::DateTime<Utc>, BatteryHistoryRecord> = HashMap::new();

    // mock data
    let yesterday = Utc::now() - chrono::Duration::days(1);
    let now: chrono::prelude::DateTime<Utc> = Utc::now();

    let data = get_data_from_csv("E:\\prophesy\\batteryreport.csv").unwrap();

    // offseting the current data by a day

    predicted_pairs.extend(data.iter().filter_map(|(x, y)| {
        if yesterday < *x && now > *x {
            let added_datetime = (x.checked_add_days(chrono::Days::new(1))).unwrap();
            Some((
                added_datetime,
                BatteryHistoryRecord {
                    capacity: y.capacity,
                    date_time: added_datetime,
                    state: y.state,
                },
            ))
        } else {
            None
        }
    }));

    Ok(predicted_pairs)
}
