use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ref input_file = args[1];
    let directions:Vec<Vec<Direction>> = parse_input(input_file);
    let mut final_code: Vec<char> = vec![];

    let mut digit_tracker = Dialpad::new();
    for direction_set in directions {
        for code in direction_set {
            digit_tracker.move_direction(code);
        }
        final_code.push(get(digit_tracker.x, digit_tracker.y));
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

const DIALPAD_CHARS:[[char; 5]; 5] = [['-', '-', '1', '-', '-'],
                                      ['-', '2', '3', '4', '-'],
                                      ['5', '6', '7', '8', '9'],
                                      ['-', 'A', 'B', 'C', '-'],
                                      ['-', '-', 'D', '-', '-']];

enum Direction { Up, Down, Right, Left }
struct Dialpad {
    x: i8,
    y: i8,
}
impl Dialpad {
    fn new() -> Dialpad {
        Dialpad{x: 0, y: 2}
    }

    fn move_direction(&mut self, direction: Direction) {
       match direction {
           Direction::Up    => if self.can_move_up() { self.x = self.x - 1 },
           Direction::Down  => if self.can_move_down() { self.x = self.x + 1 },
           Direction::Right => if self.can_move_right() { self.y = self.y + 1 },
           Direction::Left  => if self.can_move_left() { self.y = self.y - 1 },
       }
    }
    fn can_move_up(&self) -> bool {
        (self.x - 1) >= 0 && get(self.x - 1, self.y) != '-'
    }
    fn can_move_down(&self) -> bool {
        (self.x + 1) < 5 && get(self.x + 1, self.y) != '-'
    }
    fn can_move_right(&self) -> bool {
        (self.y + 1) < 5 && get(self.x, self.y + 1) != '-'
    }
    fn can_move_left(&self) -> bool {
        (self.y - 1) >= 0 && get(self.x, self.y - 1) != '-'
    }
}
fn get(x: i8, y: i8) -> char {
    DIALPAD_CHARS[x as usize][y as usize]
}
