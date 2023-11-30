use starship_battery::{
    units::{
        electric_potential::attovolt, energy::attojoule, power::attowatt, ratio::part_per_hundred,
        ElectricPotential, Energy, Power, Ratio, ThermodynamicTemperature, Time,
    },
    State, Technology,
};

use starship_battery as battery;

pub struct BatteryInfo {
    cycle_count: Option<u32>,
    state: State,
    pub state_of_charge: Ratio,
    state_of_health: Ratio,
    energy: Energy,
    energy_full_design: Energy,
    energy_full: Energy,
    energy_rate: Power,
    voltage: ElectricPotential,
    technology: Technology,
    temperature: Option<ThermodynamicTemperature>,
    vendor: String,
    model: String,
    serial_no: String,
    time_to_full: Option<Time>,
    time_to_empty: Option<Time>,
}

impl BatteryInfo {
    pub fn new() -> Self {
        let mut battery_info = BatteryInfo {
            cycle_count: None,
            state: State::Unknown,
            state_of_charge: Ratio::new::<part_per_hundred>(0.0), //maximum capacity of battery
            state_of_health: Ratio::new::<part_per_hundred>(0.0),
            energy: Energy::new::<attojoule>(0.0),
            energy_full: Energy::new::<attojoule>(0.0),
            energy_full_design: Energy::new::<attojoule>(0.0),
            energy_rate: Power::new::<attowatt>(0.0),
            voltage: ElectricPotential::new::<attovolt>(0.0),
            technology: Technology::Unknown,
            temperature: None,
            vendor: String::new(),
            model: String::new(),
            serial_no: String::new(),
            time_to_full: None,
            time_to_empty: None,
        };
        let manager = battery::Manager::new().unwrap();

        for (_idx, maybe_battery) in manager.batteries().unwrap().enumerate() {
            let battery = maybe_battery.unwrap();
            battery_info.cycle_count = battery.cycle_count();
            battery_info.state = battery.state();
            battery_info.state_of_charge = battery.state_of_charge();
            battery_info.state_of_health = battery.state_of_health();
            battery_info.energy = battery.energy();
            battery_info.energy_full = battery.energy_full();
            battery_info.energy_full_design = battery.energy_full_design();
            battery_info.energy_rate = battery.energy_rate();
            battery_info.voltage = battery.voltage();
            battery_info.technology = battery.technology();
            battery_info.temperature = battery.temperature();
            battery_info.vendor = battery.vendor().unwrap().to_string();
            battery_info.model = battery.model().unwrap().to_string();
            battery_info.serial_no = battery.serial_number().unwrap().to_string();
            battery_info.time_to_empty = battery.time_to_empty();
            battery_info.time_to_full = battery.time_to_full();
        }
        battery_info
    }
    pub fn print_info(&self) {
        println!("Cycle Count: {:?}", self.cycle_count);
        println!("State: {:?}", self.state);
        println!("State of Charge: {:?}", self.state_of_charge);
        println!("State of Health: {:?}", self.state_of_health);
        println!("Energy: {:?}", self.energy);
        println!("Energy Full: {:?}", self.energy_full);
        println!("Energy Full Design: {:?}", self.energy_full_design);
        println!("Energy Rate: {:?}", self.energy_rate);
        println!("Voltage: {:?}", self.voltage);
        println!("Technology: {:?}", self.technology);
        println!("Temperature: {:?}", self.temperature);
        println!("Vendor: {:?}", self.vendor);
        println!("Model: {:?}", self.model);
        println!("Serial No: {:?}", self.serial_no);
        println!("Time to Empty: {:?}", self.time_to_empty);
        println!("Time to Full: {:?}", self.time_to_full);
    }
    pub fn update_info(&mut self) {
        let manager = battery::Manager::new().unwrap();

        for (_idx, maybe_battery) in manager.batteries().unwrap().enumerate() {
            let battery = maybe_battery.unwrap();
            self.cycle_count = battery.cycle_count();
            self.state = battery.state();
            self.state_of_charge = battery.state_of_charge();
            self.state_of_health = battery.state_of_health();
            self.energy = battery.energy();
            self.energy_full = battery.energy_full();
            self.energy_full_design = battery.energy_full_design();
            self.energy_rate = battery.energy_rate();
            self.voltage = battery.voltage();
            self.temperature = battery.temperature();
            self.time_to_empty = battery.time_to_empty();
            self.time_to_full = battery.time_to_full();
        }
    }
}
