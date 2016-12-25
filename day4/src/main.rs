use std::env;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
struct Room {
    name: HashMap<char, u16>,
    sector_id: u32,
    checksum: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let ref input_file = args[1];
    let rooms:Vec<Room> = parse_input(input_file);
    println!("# of valid rooms: {}", valid_rooms(rooms));
}
fn parse_input(file: &String) -> Vec<Room> {
    let mut result: Vec<Room> = vec![];

    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        let mut room = Room::new();
        let data = line.unwrap();
        let mut parts:Vec<&str> = data.split('-').collect();
        let metadata = parts.pop().unwrap();
        for entry in parts {
            for c in entry.chars() {
                let char_count = room.name.entry(c).or_insert(0);
                *char_count += 1;
            }
        }
        let parts:Vec<&str> = metadata.split(|c| c == '[' || c == ']').collect();
        room.sector_id = parts[0].parse::<u32>().unwrap();
        room.checksum = String::from(parts[1]);
        result.push(room);
    }

    result
}
fn valid_rooms(rooms: Vec<Room>) -> u32 {
    rooms.iter().fold(0, |accumulator, room| {
        let mut sorted:Vec<(&char, &u16)> = room.name.iter().collect();
        sorted.sort_by(|a, b| {
            if a.1 == b.1 { a.0.cmp(b.0) }
            else { b.1.cmp(a.1) }
        });
        sorted.truncate(5);
        let first_five:String = sorted.into_iter().map(|val| *val.0).collect();

        if first_five == room.checksum { accumulator + room.sector_id }
        else { accumulator }
    })
}

impl Room {
    fn new() -> Room {
        Room {
            name: HashMap::new(),
            sector_id: 0,
            checksum: String::new(),
        }
    }
}
