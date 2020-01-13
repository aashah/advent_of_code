use std::collections::HashSet;
use std::fs;

fn parse_points(wire: &str) -> HashSet<(i32, i32)> {
    let mut cur_point = (0, 0);
    let mut points = HashSet::new();

    wire.split(",").for_each(|piece| {
        let (dir, dist_str) = piece.split_at(1);
        let dist = dist_str.parse().unwrap();

        for _ in 0..dist {
            match dir {
                "U" => cur_point.1 += 1,
                "D" => cur_point.1 -= 1,
                "L" => cur_point.0 -= 1,
                "R" => cur_point.0 += 1,
                _ => panic!("Invalid dir: {}", dir),
            }
            points.insert(cur_point);
        }
    });

    points
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let filename = "data/day3.txt";
    let input = fs::read_to_string(filename)?;

    let wires: Vec<&str> = input.split('\n').collect();

    let wire1 = parse_points(wires[0]);
    let wire2 = parse_points(wires[1]);

    let result = wire1
        .intersection(&wire2)
        .map(|point| point.0.abs() + point.1.abs())
        .min()
        .unwrap();

    println!("Result {}", result);
    Ok(())
}
