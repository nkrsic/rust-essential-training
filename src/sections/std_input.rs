use std::io;

pub fn use_stdin() {
    let mut buffer = String::new();
    println!("Enter some text");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Something weird happened reading your input!");

    let number: i32 = buffer.trim().parse().unwrap();

    println!("You entered: \"{}\"", number);
    // .parse() Returns a Result<T,E> enum
    // let number: i32 = match buffer.trim().parse::<i32>() {
    //     Ok(number) => number,
    //     Err(e) => {
    //         println!("An error occurred {}", e);
    //         return;
    //     }
    // };

    println!("You entered {} as a number", number);
}
