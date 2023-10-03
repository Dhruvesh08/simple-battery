use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct PowerSupply {
    name: String,
    r#type: String,
    status: String,
    present: bool,
    voltage_now: u32,
    current_now: i32,
    capacity: u8,
    capacity_level: String,
    temp: i32,
    technology: String,
    charge_full: u32,
    charge_now: u32,
    charge_full_design: u32,
    manufacturer: String,
}

pub trait PowerSupplyInfo {
    fn info(&self) -> PowerSupply;
    fn set_device(&mut self, device: &str);
    fn get_device(&self) -> &str;
    fn get_current(&self) -> Result<i64, std::io::Error>;
}

pub struct Battery {
    pub path: String,
    pub currnet_now: String,
}

impl PowerSupplyInfo for Battery {
    fn info(&self) -> PowerSupply {
        let mut file = File::open(&self.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut power_supply = PowerSupply {
            name: String::new(),
            r#type: String::new(),
            status: String::new(),
            present: false,
            voltage_now: 0,
            current_now: 0,
            capacity: 0,
            capacity_level: String::new(),
            temp: 0,
            technology: String::new(),
            charge_full: 0,
            charge_now: 0,
            charge_full_design: 0,
            manufacturer: String::new(),
        };

        for line in contents.lines() {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();

            match key {
                "POWER_SUPPLY_NAME" => power_supply.name = value.to_string(),
                "POWER_SUPPLY_TYPE" => power_supply.r#type = value.to_string(),
                "POWER_SUPPLY_STATUS" => power_supply.status = value.to_string(),
                "POWER_SUPPLY_PRESENT" => power_supply.present = value == "1",
                "POWER_SUPPLY_VOLTAGE_NOW" => power_supply.voltage_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CURRENT_NOW" => power_supply.current_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CAPACITY" => power_supply.capacity = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CAPACITY_LEVEL" => power_supply.capacity_level = value.to_string(),
                "POWER_SUPPLY_TEMP" => power_supply.temp = value.parse().unwrap_or(0),
                "POWER_SUPPLY_TECHNOLOGY" => power_supply.technology = value.to_string(),
                "POWER_SUPPLY_CHARGE_FULL" => power_supply.charge_full = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CHARGE_NOW" => power_supply.charge_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CHARGE_FULL_DESIGN" => {
                    power_supply.charge_full_design = value.parse().unwrap_or(0)
                }
                "POWER_SUPPLY_MANUFACTURER" => power_supply.manufacturer = value.to_string(),
                _ => {}
            }
        }

        power_supply
        
    }

    fn set_device(&mut self, device: &str) {
        self.path = device.to_owned();
    }

    fn get_device(&self) -> &str {
        &self.path
    }

    //to get current_now value read file from current_now path
    fn get_current(&self) -> Result<i64, std::io::Error> {
        let mut file = fs::File::open(&self.currnet_now)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let value: i64 = contents
            .trim()
            .parse()
            .map_err(|e: ParseIntError| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(value)
    }
}