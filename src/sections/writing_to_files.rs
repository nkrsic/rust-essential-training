use std::fs;
use std::io::prelude::*;

pub fn write_to_file() {
    let mut speech = String::new();

    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planets.txt")
        .unwrap();

    let _ = file.write(b"\nPluto");
}
