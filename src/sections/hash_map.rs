use std::collections::HashMap;

pub fn w_hash_map() {
    let mut missions_flown = HashMap::new();

    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 0);
    missions_flown.insert("Barron", 0);

    println!("\n === HashMap ===\n");
    println!("Missions flown is: {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barron"); // Kayla Barron
    let hurley_missions = missions_flown.get("Hurley"); // returns an Option-wrapped value

    // Updating hash maps

    // 1. Call insert again
    missions_flown.insert("Barron", 1);

    // 2. Insert a new key-value pair
    missions_flown.entry("Barron").or_insert(2);
    // new key
    missions_flown.entry("Stone").or_insert(2);
    // The .entry().or_insert() returns the entry
    let kayla = missions_flown.entry("Kayla").or_insert(4);
    println!("Kayla's entry is {:?}", kayla);
    // Update Kayla's entry using de-reference
    *kayla += 1;
    println!("Kayla's entry is {:?}", kayla);
}
