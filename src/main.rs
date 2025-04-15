use rust_essential_training::sections::bitwise::do_bitwise;
use rust_essential_training::sections::average::average;

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
}
