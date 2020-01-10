use std::fs::File;
use std::io::{BufRead, BufReader};

struct Fuel {
   mass: u32
}

impl Iterator for Fuel {
   type Item = u32;

   fn next(&mut self) -> Option<u32> {
      let f = fuel_required_for_mass(self.mass);

      if f == 0 {
         return None;
      }

      self.mass = f;

      Some(f)
   }
}

fn main() {
    let filename = "data/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let sum: u32 = reader.lines()
        .map(|line| line.expect("failed to parse line").parse::<u32>().unwrap())
        .map(|mass| fuel_required_for_mass(mass))
        .map(|fuel| fuel + fuel_required_for_fuel(fuel))
        .sum();


    println!("{}", sum);
}

fn fuel_required_for_mass(mass: u32) -> u32 {
    let intermediate = (f64::from(mass) / 3.0).floor() - 2.0;

    if intermediate <= 0.0 {
        return 0;
    }

    intermediate as u32
}

fn fuel_required_for_fuel(fuel: u32) -> u32 {
   Fuel{mass: fuel}.sum()
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

    #[test]
    fn fuel_required_for_fuel() {
        assert_eq!(super::fuel_required_for_fuel(2), 0);
        assert_eq!(super::fuel_required_for_fuel(654), 312);
        assert_eq!(super::fuel_required_for_fuel(33583), 16763);
    }
}

