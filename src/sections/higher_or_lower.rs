use rand::Rng;
use std::io;

pub fn play_higher_or_lower() {
    let random_number = rand::rng().gen_range(1..101);
    let mut guess: i32 = -1;
    let mut buffer = String::new();

    println!("Checking buffer!");

    while guess != random_number {
        println!("Please guess a number between 1 and 100 (inclusive)");
        buffer.clear();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading your input!");

        guess = buffer
            .trim()
            .parse()
            .expect("An error occurred parsing your input.");

        if guess == random_number {
            println!("You guessed it! It was {}", guess);
        } else if guess < random_number {
            println!("My number is greater than your guess.");
        } else {
            println!("My number is less than your guess.")
        }
    }
}
