use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "data/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let sum: u32 = reader.lines()
        .map(|line| line.expect("failed to parse line").parse::<u32>().unwrap())
        .map(|mass| fuel_required_for_mass(mass))
        .sum();

    println!("{}", sum)
}

fn fuel_required_for_mass(mass: u32) -> u32 {
    (f64::from(mass) / 3.0 - 2.0) as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn fuel_required_for_mass() {
        assert_eq!(super::fuel_required_for_mass(12), 2);
        assert_eq!(super::fuel_required_for_mass(14), 2);
        assert_eq!(super::fuel_required_for_mass(1969), 654);
        assert_eq!(super::fuel_required_for_mass(100756), 33583);
    }
}

