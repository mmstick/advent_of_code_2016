use std::cmp::Ordering::{Less, Greater};
use std::convert::From;
use std::str::Lines;

/// Contains the character as a `key` and it's frequency as the `value`
struct Frequency { key: u8, value: u8 }

/// A map of character frequencies
struct FrequencyMap { data: Vec<Frequency> }

impl FrequencyMap {
    /// If key exists, increment it, else add it to the map
    fn increment_key(&mut self, key: u8) {
        for element in &mut self.data {
            if element.key == key {
                element.value += 1;
                return
            }
        }
        self.data.push(Frequency{ key: key, value: 1 });
    }

    /// Sort the frequency map by the greater number of occurrences first, and alphabetical order second.
    fn sort(&mut self) {
        self.data.sort_by(|a, b| {
            if a.value > b.value { Less } else if a.value < b.value || a.key > b.key { Greater } else { Less }
        })
    }

    /// Collect the first five characters in the sorted frequency map as the checksum of the map.
    fn collect_checksum(&mut self) -> Vec<u8> {
        self.sort();
        self.data.iter().take(5).map(|x| x.key).collect::<Vec<u8>>()
    }
}

impl<'a> From<&'a str> for FrequencyMap {
    fn from(name: &'a str) -> FrequencyMap {
        let mut freqmap = FrequencyMap { data: Vec::new() };
        for character in name.bytes().filter(|&x| x != b'-') { freqmap.increment_key(character); }
        freqmap
    }
}

/// Take a character as a byte and wrap add the character by the alphabet. 'a' ... 'z' -> 'a' ... 'z' -> ...
fn wrap_to_char(mut character: u8, by: u32) -> char {
    for _ in 0..by { character = if character == b'z' { b'a' } else { character + 1 }; }
    character as char
}

/// Iterates through a list of encrypted rooms and returns their decrypted names and associated room numbers.
struct RoomIterator<'a> {
    lines: Lines<'a>
}

impl<'a> RoomIterator<'a> {
    fn new(input: &'a str) -> RoomIterator<'a> {
        RoomIterator { lines: input.lines() }
    }
}

impl<'a> Iterator for RoomIterator<'a> {
    type Item = (String, u32);
    fn next(&mut self) -> Option<(String, u32)> {
        loop {
            if let Some(line) = self.lines.next() {
                let (prefix, checksum) = line.split_at(line.find('[').unwrap());
                let (name, sector_id) = prefix.split_at(line.find(|x: char| x.is_numeric()).unwrap());
                let expected_checksum = checksum[1..checksum.len()-1].as_bytes();

                if FrequencyMap::from(name).collect_checksum() == expected_checksum {
                    let sector_id = sector_id.parse::<u32>().unwrap();
                    return Some((name.bytes().map(|x| {
                        if x == b'-' { ' ' } else { wrap_to_char(x, sector_id) }
                    }).collect::<String>(), sector_id));
                } else {
                    continue
                }
            } else {
                return None;
            }
        }
    }
}

fn main() {
    let inputs = include_str!("input.txt");

    let sum = RoomIterator::new(inputs).fold(0, |acc, x| acc + x.1);
    println!("There sum of valid room sector IDs is {}.", sum);

    let room = RoomIterator::new(inputs).find(|x| x.0.contains("north")).unwrap();
    println!("The north pole objects are stored in room {}", room.1)
}

#[test]
fn part_one() {
    let inputs = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]\n";
    assert_eq!(1514, RoomIterator::new(inputs).fold(0, |acc, x| acc + x.1));

    let inputs = include_str!("input.txt");
    assert_eq!(245102, RoomIterator::new(inputs).fold(0, |acc, x| acc + x.1));
}

#[test]
fn part_two() {
    let inputs = include_str!("input.txt");
    let expected = include_str!("decrypted.txt");

    let mut actual = String::new();
    for room in RoomIterator::new(inputs) {
        actual.push_str(&format!("{}[{}]\n", room.0, room.1));
    }

    for (actual, expected) in actual.lines().zip(expected.lines()) {
        assert_eq!(actual, expected);
    }

    let room = RoomIterator::new(inputs).find(|x| x.0.contains("north")).unwrap();
    assert_eq!(324, room.1);
}
