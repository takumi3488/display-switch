mod placer;
mod store;
use std::io;

use placer::Placer;
use store::{Display, DisplaySwitch};

fn main() {
    let mut display_switch = DisplaySwitch::new();
    let placer = Placer::new();
    let current: &str = &placer.current();

    if display_switch
        .displays
        .iter()
        .find(|&d| d.get_place() == current)
        .is_none()
    {
        println!("Current display place is not in the list. Enter a name for the current display:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        display_switch.add(Display::new(name.trim().to_string(), current.to_string()));
    }

    let current_index = display_switch
        .displays
        .iter()
        .position(|d| d.get_place() == current)
        .unwrap();
    let next_index = (current_index + 1) % display_switch.displays.len();
    let next = &display_switch.displays[next_index];
    placer.set(next.get_place());
    println!("Switched to display: {}", next.get_name());
}
