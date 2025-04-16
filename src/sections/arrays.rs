
pub fn some_arrays(){
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    
    let first_letter = letters[0];
    
    println!("{}", first_letter);

    let numbers: [i32; 5];
    numbers = [0,0,0,0,0];

    let numbers2: [i32; 5];
    let numbers2 = [0; 5];
    let index: usize = numbers2.len();
    // usize depends on architecture (32 vs 64 bit etc)
    println!("last number is {}", numbers[4]);

    // Multi-dimensional arrays

    let multi_dim = [[1,2,3], [4,5,6]];
    let number = multi_dim[0][1];

    // Inner arrays must have the same shape, or 
    // Rust will error/panic
    //
    // let multi_dim = [[1,2,3], [4,5,6,7]];

    // 3-dimensional array 5 x 20 x 100
    let garage: [[[i32; 100]; 20]; 5];

    // Same as above but with all elements
    // initialized to the value 11
    let garage_2 = [[[11; 100]; 20]; 5];
}