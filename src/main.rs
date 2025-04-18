use rust_essential_training::sections::average::average;
use rust_essential_training::sections::bitwise::do_bitwise;
use rust_essential_training::sections::command_line_args::use_command_line_args;
use rust_essential_training::sections::functions::say_hello;
use rust_essential_training::sections::higher_or_lower::play_higher_or_lower;
use rust_essential_training::sections::loops_challenge::loops_challenge;
use rust_essential_training::sections::reading_from_files::{check_roster, read_a_file};
use rust_essential_training::sections::std_input::use_stdin;
use rust_essential_training::sections::trim_spaces::test_trim_spaces;
use rust_essential_training::sections::writing_to_files::write_to_file;

use ::std::env;

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

    // Chapter 9
    use_command_line_args();
    read_a_file();
    write_to_file();

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 3 {
        println!("User must specify at least two args");
        return;
    }
    check_roster(&arguments[1], &arguments[2])
}
