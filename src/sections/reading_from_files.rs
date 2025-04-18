use std::fs;

pub fn read_a_file() {
    // Can also use std::path instead of raw strings
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("Contents is {}", contents);

    for line in contents.lines() {
        println!("Line is {}", line);
    }

    // Using bytes ..

    // Returns a vector of bytes
    let contents_bytes = fs::read("planets.txt").unwrap();

    println!("Contents is {:?}", contents);
}

pub fn check_roster(path: &str, name: &str) {
    let contents = fs::read_to_string(path).unwrap();

    for line in contents.lines() {
        if line == name {
            println!("The name \"{}\" is on the roster!", name);
            return;
        }
    }
    println!("The name \"{}\" was not found", name);
}
