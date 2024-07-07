mod placer;
mod store;
use std::io::{self, stdout, Write};

use placer::Placer;
use store::{Display, DisplaySwitch};

fn main() {
    let mut display_switch = DisplaySwitch::new();
    let placer = Placer::new();
    let current = Display::places_from_str(&placer.current());

    if display_switch
        .displays
        .iter()
        .find(|&d| d.get_places() == &current)
        .is_none()
    {
        print!("Current display place is not in the list. Enter a name for the current display: ");
        stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        if name.trim().is_empty() {
            print!("Name cannot be empty. Continue without saving? (y/n): ");
            stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "y" {
                return;
            }
        } else {
            let current_display = Display::new(name.trim(), &current);
            display_switch.add(&current_display);
            placer.set(&current_display.get_places_vec());
            println!("Added display: {}", current_display.get_name());
            return;
        }
    }

    let current_index = display_switch
        .displays
        .iter()
        .position(|d| d.get_places() == &current)
        .unwrap_or(display_switch.displays.len() - 1);
    let mut current_place_ids = current.iter().map(|p| p.get_id()).collect::<Vec<_>>();
    current_place_ids.sort();
    let next_index = (1..=display_switch.displays.len())
        .find(|i| {
            let places = &display_switch.displays
                [(current_index + i) % display_switch.displays.len()]
            .get_places();
            let mut place_ids = places.iter().map(|p| p.get_id()).collect::<Vec<_>>();
            place_ids.sort();
            place_ids == current_place_ids
        })
        .expect("No display place found for current display.");
    let next =
        &display_switch.displays[(current_index + next_index) % display_switch.displays.len()];
    placer.set(&next.get_places_vec());
    println!("Switched to display: {}", next.get_name());
}
