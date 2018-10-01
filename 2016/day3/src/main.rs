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

    let mut buffer:Vec<u32> = vec![];
    for line in reader.lines() {
        let data = line.expect("Bad line data");
        let side_lengths:Vec<u32> = data.split_whitespace()
            .map(|x| x.parse::<u32>().expect("not a number"))
            .collect();
        buffer.push(side_lengths[0]);
        buffer.push(side_lengths[1]);
        buffer.push(side_lengths[2]);

        if buffer.len() == 9 {
            result.push((buffer[0], buffer[3], buffer[6]));
            result.push((buffer[1], buffer[4], buffer[7]));
            result.push((buffer[2], buffer[5], buffer[8]));
            buffer.clear();
        }
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
