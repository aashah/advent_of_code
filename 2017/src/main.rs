use std::env;
use std::fs::File;
use std::io::prelude::*;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref input_file = args[1];
    let mut file = File::open(input_file).expect("Could not open input.txt");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read input.txt");

    let answer = day2::puzzle(&input);
    println!("The answer is {}", answer);
}
