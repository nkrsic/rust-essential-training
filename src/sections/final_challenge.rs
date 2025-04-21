use std::collections::HashMap;
use std::env;
use std::fs;

pub fn count_words_from_file() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("User must specify filename as first argument");
    }
    let filename = args.get(1).unwrap(); // Consider using 

    let file_contents = fs::read_to_string(filename).unwrap().to_lowercase();

    println!("=== File @ {} -- first 100 chars", filename);
    println!("{}", &file_contents[0..100]);

    let mut word_counts: HashMap<String, i32> = HashMap::new();

    for word in file_contents.split_whitespace() {
        let e = word_counts.entry(String::from(word));
        e.and_modify(|x| *x += 1).or_insert(1);
    }

    let mut kv_pair_array: Vec<(&String, &i32)> = word_counts.iter().collect();
    println!("{:?}", &kv_pair_array[1..20]);

    kv_pair_array.sort_by_key(|&(_, v)| v);

    println!("{:?}", &kv_pair_array[1..20]);

    println!("\n === Word Count Summary === \n");
    let words = kv_pair_array.len();
    for item in &kv_pair_array[(words - 5)..words] {
        println!("{:?}", item);
    }
}
