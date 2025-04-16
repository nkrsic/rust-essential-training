
pub fn infinite_loop(){
    let mut count = 0;
    
    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop, counter is: {}", count);
    println!("Result is {}", result);
}

pub fn while_loop(){
    let mut count = 0;
    let letters = ['a', 'b', 'c'];
    // NOTE: 
    // 
    // while loop cannot return a value
    while count < 10 {
        count += 1;
        println!("count is {}", count);
    } 

    count = 0;
    while count < letters.len() {
        println!("letter: {}", letters[count]);
        count += 1;
    }
}

pub fn for_loop(){
    let letters = ['a', 'b', 'c', 'd', 'e'];

    // NOTE: 
    // Requires Rust 1.53 and higher
    for item in letters {
        println!("item: {}", item);
    }

    // NOTE: 
    // For versions before 1.53, use the .iter() method
    for item in letters.iter() {
        println!("item: {}", item);
    }

    // Add index
    // 
    // Ampersand is necessary for 'e' comparison
    for (index, &item) in letters.iter().enumerate() {
        println!("item: {}", item);
        if item == 'e' {
            break;
        }
    }

    // Range-based for loop
    // 
    // Upper bound NOT included
    for number in 0..5 {
        println!("number: {}", number);
    }

}