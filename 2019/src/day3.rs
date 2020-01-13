use std::collections::HashMap;
use std::fs;

fn parse_points(wire: &str) -> HashMap<(i32, i32), u32> {
    let mut cur_point = (0, 0);
    let mut points = HashMap::new();
    let mut steps = 0;

    wire.split(",").for_each(|piece| {
        let (dir, dist_str) = piece.split_at(1);
        let dist = dist_str.parse().unwrap();

        for _ in 0..dist {
            steps += 1;
            match dir {
                "U" => cur_point.1 += 1,
                "D" => cur_point.1 -= 1,
                "L" => cur_point.0 -= 1,
                "R" => cur_point.0 += 1,
                _ => panic!("Invalid dir: {}", dir),
            }
            points.insert(cur_point, steps);
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
        .iter()
        .filter_map(|(pos, wire1_steps)| {
            wire2.get(pos).map(|wire2_steps| wire1_steps + wire2_steps)
        })
        .min();

    println!("{:?}", result);
    Ok(())
}
