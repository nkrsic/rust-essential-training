// Vectors are stored in the heap

pub fn w_vectors() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}", astronauts);

    // The .pop method actually removes the last element,
    // returns it as an option enum.
    //
    // Subsequent attempts to get it will cause array out of bounds
    // error

    let last = astronauts.pop();
    println!("last: {:?}", last);

    // The vector owns the value it's storing, need to borrow
    //
    // Will cause an error if the value has already been removed
    // and the length has shortened.
    //
    // Can use .get() instead (below)
    //
    // let third = &astronauts[2];
    // println!("third: {}", third);

    let third = astronauts.get(2);
    println!("third is {:?}", third); // returns None from Option

    // Can use the vec! macro to create new vectors

    let countdown = vec![5, 4, 3, 2, 1];
}
