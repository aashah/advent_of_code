use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "data/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let parsed_integers: Vec<u32> = reader
        .lines()
        .map(|line| line.expect("failed to parse line").parse::<u32>().unwrap())
        .collect();
    let pair: Vec<u32> = day_one_part_two(parsed_integers, 2020);
    println!(
        "{} + {} + {}= 2020; multiplied = {}",
        pair[0],
        pair[1],
        pair[2],
        pair[0] * pair[1] * pair[2]
    )
}
fn day_one_part_one(numbers: Vec<u32>, target: u32) -> Vec<u32> {
    for x in &numbers {
        for y in &numbers {
            if x + y == target {
                return vec![*x, *y];
            }
        }
    }
    return vec![];
}
fn day_one_part_two(numbers: Vec<u32>, target: u32) -> Vec<u32> {
    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if x + y + z == target {
                    return vec![*x, *y, *z];
                }
            }
        }
    }
    return vec![];
}

mod tests {
    #[test]
    fn day_one_part_one() {
        assert_eq!(
            super::day_one_part_one(vec![1721, 979, 366, 299, 675, 1456], 2020),
            vec![1721, 299]
        );
    }

    #[test]
    fn day_one_part_two() {
        assert_eq!(
            super::day_one_part_two(vec![1721, 979, 366, 299, 675, 1456], 2020),
            vec![979, 366, 675]
        );
    }
}
