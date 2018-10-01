use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

struct Movement {
    direction: char,
    steps: i32,
}
enum Direction { Up, Down, Left, Right }
fn main() {
    let args: Vec<String> = env::args().collect();

    let ref input_file = args[1];
    let movements = parse_input(input_file);

    let (final_movement, first_duplicate) = calculate_min_steps(movements);

    println!("Final # of steps {} {}", final_movement, first_duplicate);
}
fn parse_input(file: &String) -> Vec<Movement> {
    let mut data = String::new();
    let mut file = File::open(file).expect("Unable to open file");
    file.read_to_string(&mut data).expect("Unable to read string");

    data = data.replace(",", "");

    data.split(" ").map(|p| {
        let mut piece = String::from(p);
        let direction = piece.remove(0);
        let steps = piece.parse::<i32>().expect("missing number");
        Movement{
            direction: direction,
            steps: steps as i32,
        }
    }).collect()
}
fn calculate_min_steps(movements: Vec<Movement>) -> (i32, i32) {
    let mut current_direction = Direction::Up;
    let mut y: i32 = 0;
    let mut x: i32 = 0;

    let mut positions:HashSet<(i32, i32)> = HashSet::new();
    let mut found_duplicate_visit = false;
    let mut duplicate_distance = 0;
    for movement in movements.iter() {
        current_direction = change_direction(current_direction, movement.direction);
        match walk(&mut x, &mut y, &current_direction, movement.steps, &mut positions) {
            None => {},
            Some(distance) => {
                if !found_duplicate_visit {
                    found_duplicate_visit = true;
                    duplicate_distance = distance;
                }
            },
        }
    }
    (y.abs() + x.abs(), duplicate_distance)
}
fn check_second_visit(positions:&mut HashSet<(i32, i32)>, x: i32, y: i32) -> Option<(i32, i32)> {
   if positions.contains(&(x, y)) {
       return Some((x, y));
   }
   positions.insert((x, y));
   None
}
fn walk(x: &mut i32, y: &mut i32, dir: &Direction, steps: i32, positions: &mut HashSet<(i32, i32)>) -> Option<i32> {
    let mut result: Option<i32> = None;
    for _ in 0..steps {
        match *dir {
            Direction::Up => { *y = *y + 1 },
            Direction::Down => { *y = *y - 1 },
            Direction::Left => { *x = *x - 1 },
            Direction::Right => { *x = *x + 1 },
        }
        match check_second_visit(positions, *x, *y) {
            Some((x, y)) => {
                if result == None { result = Some(x.abs() + y.abs()); }
            },
            None => {},
        }
    }
    result
}
fn change_direction(current_direction: Direction, angle: char) -> Direction {
    match current_direction {
        Direction::Up => if angle == 'L' { Direction::Left } else { Direction::Right },
        Direction::Down => if angle == 'R' { Direction::Left } else { Direction::Right },
        Direction::Left => if angle == 'L' { Direction::Down } else { Direction::Up },
        Direction::Right => if angle == 'R' { Direction::Down } else { Direction::Up },
    }
}
