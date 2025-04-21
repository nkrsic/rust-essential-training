use std::fs;
use std::io;

pub fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    // These two accomplish the same thing, ? is
    // syntax sugar
    let mut s1 = fs::read_to_string(f1)?;

    let mut s2 = match fs::read_to_string(f2) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    s1.push('\n');
    s1.push_str(&s2);

    Ok(s1)
}

pub fn w_error_handling() {
    let result = fs::read_to_string("non-existing-file.txt");

    // let contents = match result {
    //     Ok(value) => value,
    //     Err(e) => String::from("File does not exist"),
    // };

    let contents = match result {
        Ok(value) => value,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => String::from("File not found!"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            _ => panic!("An error occurred"),
        },
    };

    println!("contents is: {:?}", contents);
}
