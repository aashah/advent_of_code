use std::env;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
struct Room {
    raw_name: String,
    name: HashMap<char, u16>,
    sector_id: u32,
    checksum: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let ref input_file = args[1];
    let rooms:Vec<Room> = parse_input(input_file);
    println!("# of valid rooms: {}", valid_rooms(&rooms));
    println!("sector id of north pole: {:?}", find_north_pole(&rooms).sector_id);
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
        for entry in &parts {
            for c in entry.chars() {
                let char_count = room.name.entry(c).or_insert(0);
                *char_count += 1;
            }
        }

        room.raw_name = parts.iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let parts:Vec<&str> = metadata.split(|c| c == '[' || c == ']').collect();
        room.sector_id = parts[0].parse::<u32>().unwrap();
        room.checksum = String::from(parts[1]);
        result.push(room);
    }

    result
}
fn valid_rooms(rooms: &Vec<Room>) -> u32 {
    rooms.iter().fold(0, |accumulator, room| {
        if room.valid() { accumulator + room.sector_id }
        else { accumulator }
    })
}
fn find_north_pole<'a>(rooms: &'a Vec<Room>) -> &'a Room {
    rooms.iter().filter(|r| r.valid()).find(|room| {
        let num_rotations = (room.sector_id % 26) as u8;
        let mut decoded_name = String::new();
        for c in room.raw_name.chars() {
            match c {
                'a' ... 'z' => {
                    let rotated_char = ((c as u8 -  b'a') + num_rotations) % 26 + b'a';
                    decoded_name.push(rotated_char as char);
                },
                ' ' => { decoded_name.push(' '); }
                _ => {},
            }
        }
        decoded_name == "northpole object storage"
    }).expect("couldn't find it")
}

impl Room {
    fn new() -> Room {
        Room {
            raw_name: String::new(),
            name: HashMap::new(),
            sector_id: 0,
            checksum: String::new(),
        }
    }
    fn valid(&self) -> bool {
        let mut sorted:Vec<(&char, &u16)> = self.name.iter().collect();
        sorted.sort_by(|a, b| {
            if a.1 == b.1 { a.0.cmp(b.0) }
            else { b.1.cmp(a.1) }
        });
        sorted.truncate(5);
        let first_five:String = sorted.into_iter().map(|val| *val.0).collect();
        first_five == self.checksum
    }
}
