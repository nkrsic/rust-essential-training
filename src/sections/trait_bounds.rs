use std::any;
use std::fmt;

// T must implement the Display trait from std::fmt
fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}

// T must implement the From<U> trait so we can compare the two
// U must implement Copy to avoid a move in T::from(b)
//
// The following signature is long, so we can use the 'where'
// keyword (see below) for readability
//
// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(

fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

// Return value bounds -- must be a type that implements fmt::Display
//
// NOTE: The compiler still needs to know what will be returned,
//       so you can't conditionally return different types. For that
//        you need dynamic dispatch which is an advanced topic not
//        covered in this course
fn get_displayable() -> impl fmt::Display {
    13
}

fn print_types() {
    print_type(1);
    print_type(14.5);
    print_type("thirteen");
}
