pub fn get_username(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn with_result_and_options() {
    match divide(11.0, 0.0) {
        Ok(value) => println!("The division result is {}", value),
        Err(e) => println!("{}", e),
    }

    match divide(1.0, 3.0) {
        Ok(value) => println!("The division result is {}", value),
        Err(e) => println!("{}", e),
    }

    let username = match get_username(0) {
        Some(name) => name,
        None => "".to_string(),
    };
}
