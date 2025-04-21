use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn play_higher_or_lower() {
    let random_number = rand::rng().random_range(1..101);
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

pub fn play_higher_or_lower_w_idioms() {
    let random_number = rand::rng().random_range(1..101);
    let mut guess: i32 = -1;
    let mut buffer = String::new();

    println!("Checking buffer!");

    while guess != random_number {
        println!("Please guess a number between 1 and 100 (inclusive)");
        buffer.clear();

        match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<i32>() {
                Ok(num) => {
                    guess = num;
                    match guess.cmp(&random_number) {
                        Ordering::Equal => println!("You guessed it!"),
                        Ordering::Less => println!("The number is greater than your guess."),
                        Ordering::Greater => println!("The number is less than your guess"),
                    }
                }
                Err(_) => {
                    println!("An error occurred parsing your guess");
                    continue;
                }
            },
            Err(_) => {
                println!("Failed to read your input. Guess again:");
                continue;
            }
        }

        // Put this nested inside previous match,
        // as there is nothing to parse if the first match fails
        //
        // match buffer.trim().parse::<i32>() {
        //     Ok(num) => {
        //         guess = num;
        //         match guess.cmp(&random_number) {
        //             Ordering::Equal => println!("You guessed it!"),
        //             Ordering::Less => println!("The number is greater than your guess."),
        //             Ordering::Greater => println!("The number is less than your guess"),
        //         }
        //     }
        //     Err(_) => {
        //         println!("An error occurred parsing your guess");
        //         continue;
        //     }
        // }
    }
}
