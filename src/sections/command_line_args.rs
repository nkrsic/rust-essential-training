use std::env;

pub fn use_command_line_args() {
    if env::args().len() <= 2 {
        println!("Program requires at least two arguments");
    }

    for (index, argument) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument)
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
