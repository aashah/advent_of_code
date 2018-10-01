use std::fs::File;
use std::io::prelude::*;

extern crate day5;
extern crate md5;

fn main() {
    let mut file = File::open("src/input.txt").expect("Could not open src/input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read file");
    let len = input.len();
    input.truncate(len - 1);
    println!("The input is {}", input);
    let answer = day5::puzzle(&input);
    println!("The answer is {}", answer);
}
