mod store;
use store::DisplaySwitch;

fn main() {
    let display_switch = DisplaySwitch::new();
    println!("{:?}", display_switch);
}
