
pub fn say_hello(){
    println!("Hello!");
    say_a_number(13);
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    // The compiler sees say_the_sum(x,y) 
    // and automatically sets the type of x, y 
    // to be u8. 
    //
    // Subsequent call to say_a_number(x) without
    // casting will yield an error: 
    // 
    // say_a_number(x) // This should cause an error without cast to i32
}

pub fn say_a_number(number: i32){
    println!("Your number was: {}", number);
}

pub fn say_the_sum(a: u8, b: u8){
    let sum = a + b;
    println!("The sum is: {}", sum);
}