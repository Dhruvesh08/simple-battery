mod power;

pub use power::{Battery, PowerSupply, PowerSupplyInfo};

fn main() {
    let battery = Battery {
        path: "/sys/class/power_supply/bq27441-0/uevent".to_string(),
        currnet_now: "/sys/class/power_supply/bq27441-0/current_now".to_string(),
    };
    let power_supply_info = battery.info();
    println!("{:#?}", power_supply_info);

    println!("Battery Current: {}", battery.get_current().unwrap());
}