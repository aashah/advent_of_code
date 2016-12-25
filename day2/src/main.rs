use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ref input_file = args[1];
    let directions:Vec<Vec<Direction>> = parse_input(input_file);
    let mut final_code: Vec<u8> = vec![];

    let mut digit_tracker = Dialpad::new();
    for direction_set in directions {
        println!("Starting at {}", digit_tracker.digit);
        for code in direction_set {
            digit_tracker.move_direction(code);
        }
        final_code.push(digit_tracker.digit as u8);
    }

    println!("Final code: {:?}", final_code);
}
fn parse_input(file: &String) -> Vec<Vec<Direction>> {
    let mut result: Vec<Vec<Direction>> = vec![];

    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let data = line.expect("Bad line data");
        result.push(data.chars().map(|chr| {
            match chr {
                'R' => Direction::Right,
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                _ => { panic!("bad input") },
            }
        }).collect());
    }
    result
}

enum Direction { Up, Down, Right, Left }
struct Dialpad {
    digit: i8,
}
impl Dialpad {
    fn new() -> Dialpad {
        Dialpad{digit: 5}
    }

    fn move_direction(&mut self, direction: Direction) {
       let mut row = (((self.digit - 1) as f32) / 3.).floor() as i8;
       let mut col = (self.digit - 1) % 3;

       match direction {
           Direction::Up    => if row > 0 { row = row - 1 },
           Direction::Down  => if row < 2 { row = row + 1 },
           Direction::Left  => if col > 0 { col = col - 1 } ,
           Direction::Right => if col < 2 { col = col + 1 },
       }
       self.digit = row * 3 + (col + 1);
    }
}
