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
    pub fn new(name: &str, places: &Vec<Place>) -> Display {
        Display {
            name: name.to_string(),
            places: places.clone(),
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
                        "id:{} res:{} hz:{} color_depth:{} enabled:{} scaling:{} origin:{} degree:{}",
                        p.id,
                        format!("{}x{}", p.res.0, p.res.1),
                        p.hz,
                        p.color_depth,
                        p.enabled,
                        p.scaling,
                        format!("({},{})", p.origin.0, p.origin.1),
                        p.degree
                    )
                })
                .collect::<Vec<_>>()
    }

    pub fn places_from_str(s: &str) -> Vec<Place> {
        let ptn = Regex::new(r#"id:(?<id>[0-9A-Z]{8}-([0-9A-Z]{4}-){3}[0-9A-Z]{12}) res:(?<res>[0-9]+x[0-9]+) hz:(?<hz>[0-9.]+) color_depth:(?<color_depth>[0-9]+) enabled:(?<enabled>true|false) scaling:(?<scaling>\S+) origin:(?<origin>\(-?[0-9]+,-?[0-9]+\)) degree:(?<degree>-?[0-9]+)"#).unwrap();
        s.split("\" \"")
            .map(|s| s.replace("\"", ""))
            .map(|s| {
                let caps = ptn
                    .captures(&s)
                    .expect("Unssupported format from displayplacer");
                Place {
                    id: caps["id"].to_string(),
                    res: {
                        let res: Vec<&str> = caps["res"].split("x").collect();
                        (res[0].parse().unwrap(), res[1].parse().unwrap())
                    },
                    hz: caps["hz"].parse().unwrap(),
                    color_depth: caps["color_depth"].parse().unwrap(),
                    enabled: caps["enabled"] == *"true",
                    scaling: caps["scaling"].to_string(),
                    origin: {
                        let origin: Vec<&str> = caps["origin"]
                            .trim_matches(|c| c == '(' || c == ')')
                            .split(',')
                            .collect();
                        (origin[0].parse().unwrap(), origin[1].parse().unwrap())
                    },
                    degree: caps["degree"].parse().unwrap(),
                }
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplaySwitch {
    pub displays: Vec<Display>,
}

impl DisplaySwitch {
    pub fn new() -> DisplaySwitch {
        let config_file_path = home_dir().unwrap().join(".display-switch.json");
        let config_file = match File::open(&config_file_path) {
            Ok(file) => file,
            Err(_) => File::create(&config_file_path).unwrap(),
        };
        let displays: Vec<Display> = serde_json::from_reader(config_file).unwrap_or(vec![]);
        DisplaySwitch { displays }
    }

    fn save(&self) {
        let config_file_path = home_dir().unwrap().join(".display-switch.json");
        let config_file = File::create(&config_file_path).unwrap();
        serde_json::to_writer_pretty(config_file, &self.displays).unwrap();
    }

    pub fn add(&mut self, display: &Display) {
        self.displays.push(display.clone());
        self.save();
    }
}
