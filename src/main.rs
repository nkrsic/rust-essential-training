use ::std::env;
use rust_essential_training::sections::average::average;
use rust_essential_training::sections::bitwise::do_bitwise;
use rust_essential_training::sections::box_datatype::do_something_w_box;
use rust_essential_training::sections::command_line_args::use_command_line_args;
use rust_essential_training::sections::default_trait_impl::print_space_structs_default;
use rust_essential_training::sections::enums::with_enums;
use rust_essential_training::sections::final_challenge::count_words_from_file;
use rust_essential_training::sections::functions::say_hello;
use rust_essential_training::sections::generic_structs::do_something_w_generic;
use rust_essential_training::sections::higher_or_lower::play_higher_or_lower;
use rust_essential_training::sections::higher_or_lower::play_higher_or_lower_w_idioms;
use rust_essential_training::sections::implement_display::with_implement_display;
use rust_essential_training::sections::loops_challenge::loops_challenge;
use rust_essential_training::sections::options::with_result_and_options;
use rust_essential_training::sections::reading_from_files::{check_roster, read_a_file};
use rust_essential_training::sections::std_input::use_stdin;
use rust_essential_training::sections::sum_boxes::sum_w_boxes;
use rust_essential_training::sections::traits::print_space_structs;
use rust_essential_training::sections::trim_spaces::test_trim_spaces;
use rust_essential_training::sections::writing_to_files::write_to_file;

#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name // return a slice
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    // Associate function, not a method
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

// Tuple Structs

struct Color(u8, u8, u8); // RGB

struct Point(u8, u8, u8); // RGB

fn main() {
    // Optional positional
    let a: f32 = 1.0;
    let b: f64 = 10.0;
    let c: i32 = 3;
    println!("a is {0} and c is {1}", a, c);

    do_bitwise();
    let avg = average();

    assert_eq!(avg, 45.1);
    println!("{}", avg);

    // functions
    //

    // loops challenge
    loops_challenge();

    // trim spaces challenge

    test_trim_spaces();

    use_stdin();

    // Chapter 8 -- comment out to skip guessing game
    // play_higher_or_lower();
    // play_higher_or_lower_w_idioms();

    // Chapter 9
    use_command_line_args();
    read_a_file();
    write_to_file();

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 3 {
        println!("User must specify at least two args");
        return;
    }

    // Comment to skip check_roster exercise
    //check_roster(&arguments[1], &arguments[2])

    with_result_and_options();

    // Chaper 10 - Structs

    let mut vehicle = Shuttle {
        name: String::from("Endeavor"),
        crew_size: 7,
        propellant: 835958.0,
    };

    let mut vehicle2 = Shuttle {
        // the string from 'vehicle' lives on the heap can only have one owner
        name: "Discovery".to_string(),
        ..vehicle // copy the rest of fields from 'vehicle' (above)
    };
    vehicle2.crew_size = 6;

    let vehicle3 = Shuttle {
        ..vehicle.clone() // use .clone() to copy all of the struct
    };

    // Using an associated function, Shuttle::new()
    let vehicle4 = Shuttle::new("Endeavor");

    println!("Name is: {}", vehicle.name);
    vehicle.name = "Atlantis".to_string();
    println!("{:?}", vehicle);
    println!("{:?}", vehicle2);
    println!("{:?}", vehicle3);

    vehicle2.add_fuel(1000.0);
    println!("{:?}", vehicle2);

    // Tuple structs

    let red = Color(255, 0, 0);
    let x_basis_vector = Point(1, 0, 0);

    fn get_y(p: Point) -> u8 {
        p.1
    }

    fn get_red_channel(c: Color) -> u8 {
        c.0
    }

    get_y(x_basis_vector); // Should return 0
    get_red_channel(red); // Should return 255
    // NOTE:
    // Even though the two structs share the same shape
    // you cannot use the same functions interchangeably,
    // you'd get a compiler error

    // Chapter 11.      Generic Types
    do_something_w_box();
    sum_w_boxes();

    // Chapter 12
    print_space_structs();
    print_space_structs_default();
    with_implement_display();

    // Chapter 13
    with_enums();

    // Final Challenge
    count_words_from_file();
}
