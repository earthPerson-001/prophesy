use battery_data_analysis::BatteryHistoryRecord;
use chrono::Utc;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

use reqwest;

use crate::utils::battery_realtime_data::BatteryInfo;
use battery_data_analysis::ChargeState;

const MAX_NUMBER_OF_PREDICTIONS: u8 = 10;

// Deserializable
#[derive(Deserialize)]
struct Data {
    array: Vec<Vec<f32>>,
}

pub fn get_predicted_data(
) -> Result<HashMap<chrono::DateTime<Utc>, BatteryHistoryRecord>, Box<dyn Error>> {
    let mut predicted_pairs: HashMap<chrono::DateTime<Utc>, BatteryHistoryRecord> = HashMap::new();
    let now: chrono::prelude::DateTime<Utc> = Utc::now();

    // voltage,
    // energy,
    // enry_rate,
    // time diff,
    // battery state
    let battery_status = BatteryInfo::new();
    // let data = get_data_from_csv("C:\\Users\\sunny\\Desktop\\5th Sem\\prophesy\\batteryreport..csv").unwrap();
    let mut current_capacity = battery_status.energy.value;
    const TIME_IN_MINS: i64 = 60;
    let mut i = 1;

    let mut current_prediction_number = 0;

    while (current_prediction_number < MAX_NUMBER_OF_PREDICTIONS)
        && !(current_capacity < 0.0 || current_capacity > battery_status.energy_full.value)
    {
        // println!("{:?}",);
        let x_log = [[
            battery_status.voltage.abs().value,
            battery_status.energy.abs().value,
            battery_status.energy_rate.abs().value,
            (i * TIME_IN_MINS * 60) as f32,
            if battery_status.state.to_string().eq("discharging") {
                -1.0
            } else if battery_status.state.to_string().eq("charging") {
                1.0
            } else {
                0.0
            },
        ]];

        let url = format!("http://localhost:5000/predict?x_log={:?}", x_log);
        let res = reqwest::blocking::get(url)?;
        let body = res.json::<Data>()?;
        println!("body:{:?}", body.array);
        
        current_capacity = battery_status.energy.value + body.array[0][0];
        
        predicted_pairs.insert(
            now + chrono::Duration::minutes(i * TIME_IN_MINS),
            BatteryHistoryRecord {
                capacity: (current_capacity / 3.6) as i32, // converting from Energy[J] to Energy[mWH]
                date_time: now + chrono::Duration::minutes(i * TIME_IN_MINS),
                state: if battery_status.state.to_string().eq("discharging") {
                    ChargeState::Discharging
                } else if battery_status.state.to_string().eq("charging") {
                    ChargeState::Charging
                } else {
                    ChargeState::Unknown
                },
            },
        );

        println!(
            "{:?} full, {:?} current",
            battery_status.energy_full.value, current_capacity
        );
        i += 1;
        current_prediction_number += 1;
    }

    println!("{:?}", predicted_pairs);

    Ok(predicted_pairs)
}
