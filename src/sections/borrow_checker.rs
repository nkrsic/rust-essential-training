fn w_borrow_checker() {
    let propellant: &String;
    {
        let rp1 = String::from("RP-1");
        let propellant = &rp1;
        // If this print statement is put outside this block,
        // we'd get a compiler error from the borrow checker
        println!("propellant is {}", propellant);
    }
}
