
fn if_statements(){
    let x = 3;
    let y = 5;
    let make_z_odd = true;
    let z = if make_z_odd {1} else {2};

    if x == 3 {
        println!("x is indeed equal to {}", x);
    }

    if true {
        println!("We're in the tautology 'if true' ");
    }

    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }


}