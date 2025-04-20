#[Derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64,
}

pub fn space_w_derive_traits() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };

    println!("hubble == gps is {}", hubble == gps);
    println!("hubble == gps is {}", hubble > gps);
}
