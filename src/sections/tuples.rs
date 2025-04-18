pub fn work_with_tuples() {
    // Tuples can store more than one type

    #[allow(clippy::approx_constant)]
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    // Accessing tuples, indexes start at 0
    let first_item = stuff.0;
    println!("First item: {}", first_item);

    stuff.0 += 3;
    println!("First item: {}", first_item);

    // De-structuring tuples
    let (a, b, c) = stuff;
    println!("b is: {}", b)
}
