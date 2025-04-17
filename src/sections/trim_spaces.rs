

pub fn test_trim_spaces(){
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = "   We need more space.";
    assert_eq!(trim_spaces(test2), "We need more space.");

    let test3 = "We need more space.   ";
    assert_eq!(trim_spaces(test3), "We need more space.");
    
    let test4 = "   We need more space.     ";
    assert_eq!(trim_spaces(test4), "We need more space.");
    
    let test5 = "  ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 😀 ";
    assert_eq!(trim_spaces(test7), "😀");

    println!("=== All tests in trim_spaces() passed!");
}

pub fn trim_spaces(s: &str) -> &str{

    let mut start: usize = 0;
    for (idx, character) in s.chars().enumerate() {
        if character != ' ' {
            start = idx;
            break;
        }
    }

    let mut end: usize = 0;
    for (idx, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - idx;
            break;
        }
    }
    
    &s[start..end] 
}