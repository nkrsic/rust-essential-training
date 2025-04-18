use rust_essential_training::sections::average::average;
use rust_essential_training::sections::bitwise::do_bitwise;
use rust_essential_training::sections::functions::say_hello;
use rust_essential_training::sections::higher_or_lower::play_higher_or_lower;
use rust_essential_training::sections::loops_challenge::loops_challenge;
use rust_essential_training::sections::std_input::use_stdin;
use rust_essential_training::sections::trim_spaces::test_trim_spaces;

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

    play_higher_or_lower();
}
