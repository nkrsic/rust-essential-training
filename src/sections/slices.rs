
pub fn slices(){
    let message = String::from("Greetings from Earth!");
    let str_slice = &message[15..15+5];
    println!("last word is {}", str_slice);
}