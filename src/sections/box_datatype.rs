use std::mem;

pub struct Shuttle {
    pub name: String,
    pub crew_size: u8,
    pub propellant: f64,
}

pub fn do_something_w_box() {
    let vehicle = Shuttle {
        name: "Endeavor".to_string(),
        crew_size: 7,
        propellant: 1300.0,
    };

    println!("Vehicle size on stack: {}", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle); // new ownership of struct

    println!(
        "Vehicle size on stack: {}",
        mem::size_of_val(&boxed_vehicle)
    );
    println!(
        "Vehicle size on heap: {}",
        mem::size_of_val(&*boxed_vehicle)
    );

    // Pass ownership back to the stack
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!(
        "Unboxed Vehicle size on stack: {}",
        mem::size_of_val(&unboxed_vehicle)
    );
}
