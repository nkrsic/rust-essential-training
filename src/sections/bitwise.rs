

pub fn do_bitwise(){
    let int_value = 0b1111_0101; // defaults to i32
    let value: u8 = 0b1111_0101u8; // note the u8 suffix
    
    println!("value is {}", int_value);
    println!("value is {:08b}", int_value); // b means binary bits, 0 means leading 0s, 8 means 8 bits
}


