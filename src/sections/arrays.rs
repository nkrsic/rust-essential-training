
pub fn some_arrays(){
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    
    let first_letter = letters[0];
    
    println!("{}", first_letter);

    let numbers: [i32; 5];
    numbers = [0,0,0,0,0];

    let numbers2 = [i32; 5];
    let numbers2 = [0; 5];
    let index: usize = numbers2.len();
    // usize depends on architecture (32 vs 64 bit etc)
    println!("last number is {}", numbers[4]);
}