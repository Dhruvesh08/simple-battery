mod power;

pub use power::{Battery, PowerSupply, PowerSupplyInfo};

fn main() {
    let battery = Battery {
        path: "/sys/class/power_supply/bq27441-0/uevent".to_string(),
        currnet_now: "/sys/class/power_supply/bq27441-0/current_now".to_string(),
    };
    println!("Battery Info: {}", battery.info());
    println!("Battery Path: {}", battery.get_device());
    println!("Battery Current: {}", battery.get_current().unwrap());
}