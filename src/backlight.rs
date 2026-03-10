use gtk4::{Box as GtkBox, Label, Orientation};
use gtk4::prelude::{BoxExt};
use std::fs;

const BACKLIGHT: &str = "/sys/class/backlight/";

pub fn create_widget() -> GtkBox {
    let devices:Vec<String> = get_devices();
    let max_brightness:u32 = get_max_brightness(&devices[0]);

    let container = GtkBox::new(Orientation::Horizontal, 0);
    let label = Label::new(Some(max_brightness.to_string().as_str()));
    container.append(&label);

    container
}

fn get_devices() -> Vec<String> {
    let mut devices: Vec<String> = Vec::new();

    let entries = fs::read_dir(BACKLIGHT).unwrap();
    for entry in entries {
        devices.push(entry.unwrap().file_name().display().to_string());
    }

    devices
}

fn get_max_brightness(device: &str) -> u32 {
    fs::read_to_string(format!("{}{}/max_brightness", BACKLIGHT, device))
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap()
}