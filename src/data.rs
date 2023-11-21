
use starship_battery as battery;

use battery::{units::{Ratio, Energy}, State, Battery, Manager};
use uom::si::{Quantity, Dimension};


pub struct BatteryInfo{
   cycle_count: Option<u32> ,
   state: State,
   // energy:Energy
}

impl BatteryInfo 
{

   pub fn new()->Self {
     
      let mut battery_info= BatteryInfo{
         cycle_count:None,
         state:State::Unknown,
         // energy:Quantity::<Dimension::J,>new()
      };
      let manager = Manager::new().unwrap();

      for (idx, maybe_battery) in manager.batteries().unwrap().enumerate() {
      let battery=maybe_battery.unwrap();
      battery_info.cycle_count=battery.cycle_count();
      battery_info.state=battery.state();
      // battery_info.energy= battery.energy();
      
    };
    battery_info
   

      
   }  
   pub fn print_info(&self){
         println!("The battery cycle count is:{:?}",self.cycle_count);
         println!("The current battery state is:{:?}",self.state);

      }
}