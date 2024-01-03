use battery_data_analysis::BatteryHistoryRecord;
use chrono::Utc;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

use reqwest;

use crate::utils::battery_realtime_data::BatteryInfo;
use battery_data_analysis::ChargeState;

const MAX_NUMBER_OF_PREDICTIONS: i32 = 10;
const TIME_IN_MINS: i64 = 60;
const JOULES_TO_MWHR: f32 = 3.6; // divide by this to convert from Joules to mWH

// Deserializable
#[derive(Deserialize)]
struct Data {
    array: Vec<Vec<f32>>,
}

pub fn get_predicted_data(
    use_neural_network_strictly: bool,
) -> Result<HashMap<chrono::DateTime<Utc>, BatteryHistoryRecord>, Box<dyn Error>> {
    let mut predicted_pairs: HashMap<chrono::DateTime<Utc>, BatteryHistoryRecord> = HashMap::new();
    let now: chrono::prelude::DateTime<Utc> = Utc::now();

    // voltage,
    // energy,
    // enry_rate,
    // time diff,
    // battery state
    let battery_status = BatteryInfo::new();

    // predicting all at once
    // the tiem differences is in seconds
    let time_differences = (0..=MAX_NUMBER_OF_PREDICTIONS)
        .map(|i| (i as i64 * TIME_IN_MINS * 60) as f32)
        .collect::<Vec<f32>>();

    // get predictions for those time differences
    let mut x_log: Vec<[f32; 5]> = Vec::new();
    for time_difference in time_differences.iter() {
        x_log.push([
            battery_status.voltage.abs().value,
            battery_status.energy.abs().value,
            battery_status.energy_rate.abs().value,
            *time_difference,
            if battery_status.state.to_string().eq("discharging") {
                -1.0
            } else if battery_status.state.to_string().eq("charging") {
                1.0
            } else {
                0.0
            },
        ]);
    }

    let url = format!(
        "http://localhost:5000/predict?x_log={:?}&method={}",
        x_log.as_slice(),
        if use_neural_network_strictly {
            "neural_network_only"
        } else {
            "any"
        }
    );
    println!("Sent request: {:?}", url);
    let res = reqwest::blocking::get(url)?;
    let body = res.json::<Data>()?;
    println!("body:{:?}", body.array);

    for (i, diff) in time_differences.iter().enumerate() {
        let current_capacity = battery_status.energy.value + body.array[i][0];
        predicted_pairs.insert(
            now + chrono::Duration::minutes(*diff as i64 / 60), // diff is in seconds
            BatteryHistoryRecord {
                capacity: (current_capacity / JOULES_TO_MWHR) as i32, // converting from Energy[J] to Energy[mWH]
                date_time: now + chrono::Duration::minutes(*diff as i64 / 60),
                state: if battery_status.state.to_string().eq("discharging") {
                    ChargeState::Discharging
                } else if battery_status.state.to_string().eq("charging") {
                    ChargeState::Charging
                } else {
                    ChargeState::Unknown
                },
            },
        );
    }

    println!("{:?}", predicted_pairs);

    Ok(predicted_pairs)
}
