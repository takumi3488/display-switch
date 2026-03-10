use std::fs::File;

use dir::home_dir;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Display {
    name: String,
    places: Vec<Place>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Place {
    id: String,
    res: (u32, u32),
    hz: f32,
    color_depth: u32,
    enabled: bool,
    scaling: String,
    origin: (i32, i32),
    degree: i32,
}

impl Place {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Display {
    pub fn new(name: &str, places: &[Place]) -> Display {
        Display {
            name: name.to_string(),
            places: places.to_vec(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_places(&self) -> &Vec<Place> {
        &self.places
    }

    pub fn get_places_vec(&self) -> Vec<String> {
        self.places
            .iter()
            .map(|p| {
                format!(
                    "id:{} res:{}x{} hz:{} color_depth:{} enabled:{} scaling:{} origin:({},{}) degree:{}",
                    p.id,
                    p.res.0,
                    p.res.1,
                    p.hz,
                    p.color_depth,
                    p.enabled,
                    p.scaling,
                    p.origin.0,
                    p.origin.1,
                    p.degree
                )
            })
            .collect::<Vec<_>>()
    }

    pub fn places_from_str(s: &str) -> Result<Vec<Place>, Box<dyn std::error::Error>> {
        let ptn = Regex::new(r"id:(?<id>[0-9A-Z]{8}-([0-9A-Z]{4}-){3}[0-9A-Z]{12}) res:(?<res>[0-9]+x[0-9]+) hz:(?<hz>[0-9.]+) color_depth:(?<color_depth>[0-9]+) enabled:(?<enabled>true|false) scaling:(?<scaling>\S+) origin:(?<origin>\(-?[0-9]+,-?[0-9]+\)) degree:(?<degree>-?[0-9]+)")?;
        s.split("\" \"")
            .map(|s| s.replace('"', ""))
            .map(|s| {
                let caps = ptn
                    .captures(&s)
                    .ok_or("Unsupported format from displayplacer")?;
                let res: Vec<&str> = caps["res"].split('x').collect();
                let origin_str = caps["origin"]
                    .trim_matches(|c| c == '(' || c == ')')
                    .to_string();
                let origin: Vec<&str> = origin_str.split(',').collect();
                Ok(Place {
                    id: caps["id"].to_string(),
                    res: (res[0].parse()?, res[1].parse()?),
                    hz: caps["hz"].parse()?,
                    color_depth: caps["color_depth"].parse()?,
                    enabled: caps["enabled"] == *"true",
                    scaling: caps["scaling"].to_string(),
                    origin: (origin[0].parse()?, origin[1].parse()?),
                    degree: caps["degree"].parse()?,
                })
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplaySwitch {
    pub displays: Vec<Display>,
}

impl DisplaySwitch {
    pub fn new() -> Result<DisplaySwitch, Box<dyn std::error::Error>> {
        let config_file_path = home_dir()
            .ok_or("Failed to get home directory")?
            .join(".display-switch.json");
        let config_file = match File::open(&config_file_path) {
            Ok(file) => file,
            Err(_) => File::create(&config_file_path)?,
        };
        let displays: Vec<Display> = serde_json::from_reader(config_file).unwrap_or_default();
        Ok(DisplaySwitch { displays })
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_file_path = home_dir()
            .ok_or("Failed to get home directory")?
            .join(".display-switch.json");
        let config_file = File::create(&config_file_path)?;
        serde_json::to_writer_pretty(config_file, &self.displays)?;
        Ok(())
    }

    pub fn add(&mut self, display: &Display) -> Result<(), Box<dyn std::error::Error>> {
        self.displays.push(display.clone());
        self.save()
    }
}
