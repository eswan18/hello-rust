mod median;
mod piglatin;
use rand::Rng;
use crate::median::{find_median, find_mode};
use crate::piglatin::make_piglatin;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a single argument");
    }
    match args[1].as_str() {
        "median" => test_median_and_mode(),
        "mode" => test_median_and_mode(),
        "piglatin" => test_piglatin(),
        _ => panic!("Please provide a valid argument"),
    }
}

fn test_median_and_mode() {
    let mut test_vecs: Vec<Vec<i32>> = Vec::new();
    for _ in 0..10 {
        let length = rand::thread_rng().gen_range(0..10);
        let mut v = Vec::new();
        for _ in 0..length {
            let num = rand::thread_rng().gen_range(0..100);
            v.push(num);
        }
        test_vecs.push(v);
    }

    for v in test_vecs {
        println!("Vector: {:?}", v);
        let median = find_median(&v);
        println!("Median: {:?}", median);
        let mode = find_mode(&v);
        println!("Mode: {:?}", mode);
    }
}

fn test_piglatin() {
    let words = vec!("first", "apple", "banana", "cherry", "eat", "igloo", "orange", "umbrella");
    for word in words {
        println!("{} -> {:?}", word, make_piglatin(word));
    }
}