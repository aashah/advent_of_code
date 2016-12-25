use std::cmp::max;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ref input_file = args[1];
    let triangles:Vec<(u32, u32, u32)> = parse_input(input_file);

    println!("# of incorrect triangles: {}", count_valid_triangles(triangles));
}
fn parse_input(file: &String) -> Vec<(u32, u32, u32)> {
    let mut result: Vec<(u32, u32, u32)> = vec![];

    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let data = line.expect("Bad line data");
        let side_lengths:Vec<u32> = data.split_whitespace()
            .map(|x| x.parse::<u32>().expect("not a number"))
            .collect();
        result.push((side_lengths[0], side_lengths[1], side_lengths[2]));
    }
    result
}
fn count_valid_triangles(triangles: Vec<(u32, u32, u32)>) -> u32 {
    triangles.iter().filter(|tri| {
        let sum = tri.0 + tri.1 + tri.2;
        let max = max(max(tri.0, tri.1), tri.2);
        sum - max > max
    }).count() as u32
}
