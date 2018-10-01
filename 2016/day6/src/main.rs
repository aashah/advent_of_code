use std::fs::File;
use std::io::prelude::*;

extern crate day6;

fn main() {
    let mut file = File::open("src/input.txt").expect("Could not open src/input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read file");
    let input = input.trim();
    println!("The input is {}", input);
    let answer = day6::puzzle(&input);
    println!("The answer is {}", answer);
}
