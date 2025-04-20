use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
struct Satellite {
    name: String,
    velocity: f64,
}

impl Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "name: {} velocity: {}", self.name, self.velocity)
    }
}

pub fn with_implement_display() {
    let hubble = Satellite {
        name: "Hubble Space Telescope".to_string(),
        velocity: 234.8,
    };

    println!("hubble is {}", hubble);
}
