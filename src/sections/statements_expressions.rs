
// let statement = 1;
// Expressions do not have a semi-colon, ; , at the end

fn square(x: i32) -> i32 {
    x * x // note this is an expression, no ;
}

fn square_early(x: i32) -> i32 {
    println!("Squaring {}..", x);
    return x * x;
    println!("End of function!");
}

fn square_return_tuple(x: i32) -> (i32, i32) {
    (x, x*x)
}

// Unit datatype (returning 'void' in other languages)
// 
// The following explicit return value is not necessary, 
// but there for completeness
fn return_unit(a: i32) -> (){
    println!("This function returns the unit, () or empty tuple, void");
}

