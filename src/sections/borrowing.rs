
pub fn borrow_examples(){
    let mut rocket_fuel = String::from("RP-1");

    // String value gets moved into the function and
    // passed back out -- how we'd do it without 
    // borrowing
    let (mut rocket_fuel, length) = process_fuel(rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);

    // Now we use the borrow version, using the & borrow
    // operator
    let length_borrow = process_fuel_w_borrow(&rocket_fuel);

    // NOTE:
    //
    // Once you create a mutable reference to an object, you cannot
    // create any further references to it in the same scope (mutable or 
    // non-mutable )
    let length_mut_borrow = process_fuel_w_mutable_borrow(&mut rocket_fuel);
}

pub fn process_fuel(propellant: String) -> (String, usize) {
    println!("Processing propellant {}...", propellant);
    let length = propellant.len();
    (propellant, length)
}

pub fn process_fuel_w_borrow(propellant: &String) -> usize {
    println!("Processing propellant {}...", propellant);
    let length = propellant.len();
    length
}

pub fn process_fuel_w_mutable_borrow(propellant: &mut String) -> usize {
    println!("Processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}

