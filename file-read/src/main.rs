// #![feature(link_args)]

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

// #![link_args = ""]
fn main() {
    let mut file = File::open("data.txt").expect("Cannot open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read file");

    let mut counter = HashMap::new();
    for line in contents.lines() {
        for word in line.trim().split(" ") {
            if word == "" {
                continue;
            }
            let counter = counter.entry(word).or_insert(0);
            *counter += 1;
        }
    }

    println!("Counter example:");
    for (word, n) in &counter {
        println!("{}: {}", word, n);
    }
}
