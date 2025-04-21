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

    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number2 = countdown.get(4);
    let number = number.unwrap_or(&0) + 1; // shadowed, must unwrap Option

    // NOTE: Warning says this re-implements unwrap_or() (See above)
    let number = match number2 {
        Some(n) => n,
        None => &0,
    };
    // Should return None
    println!("number is {:?}", number);

    // if let syntax replaces match syntax below

    let m = Some(13);
    match m {
        Some(13) => println!("Got 13"),
        _ => (),
    }

    // Note pattern and value switch spots
    if let Some(13) = m {
        println!("Thirteen!")
    }
}
