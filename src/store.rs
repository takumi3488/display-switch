use std::fs::File;

use dir::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Display {
    name: String,
    place: String,
}

impl Display {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_place(&self) -> &str {
        &self.place
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplaySwitch {
    pub displays: Vec<Display>,
}

impl DisplaySwitch {
    pub fn new() -> DisplaySwitch {
        let config_file_path = home_dir().unwrap().join(".display-switch.json");
        let config_file =
            File::open(&config_file_path).unwrap_or(File::create(&config_file_path).unwrap());
        let displays: Vec<Display> = serde_json::from_reader(config_file).unwrap_or(vec![]);
        DisplaySwitch { displays }
    }
}
