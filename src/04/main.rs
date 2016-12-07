#![feature(alloc_system)]
extern crate alloc_system;
extern crate arrayvec;

use std::cmp::Ordering::{Less, Greater};
use std::convert::From;
use std::str::Lines;

// Used to eliminate dynamic heap allocations by allocating a fixed-sized vector on the stack.
use arrayvec::ArrayVec;

/// Contains the character as a `key` and it's frequency as the `value`
struct Frequency { key: u8, value: u8 }

/// A map of character frequencies
struct FrequencyMap { data: ArrayVec<[Frequency; 26]> }

impl FrequencyMap {
    /// Increment a given character in the map.
    /// The character's index is guaranteed to be the character's integer representation minus 97.
    fn increment_key(&mut self, key: u8) { self.data[(key - 97) as usize].value += 1; }

    /// Sort the frequency map by the greater number of occurrences first, and alphabetical order second.
    fn sort(&mut self) {
        self.data.sort_by(|a, b| {
            if a.value > b.value { Less } else if a.value < b.value || a.key > b.key { Greater } else { Less }
        })
    }

    /// Collect the first five characters in the sorted frequency map as the checksum of the map.
    fn collect_checksum(&mut self) -> ArrayVec<[u8; 5]> {
        self.sort();
        self.data.iter().take(5).map(|x| x.key).collect::<ArrayVec<[u8; 5]>>()
    }
}

impl<'a> From<&'a str> for FrequencyMap {
    fn from(name: &'a str) -> FrequencyMap {
        let mut freqmap = FrequencyMap {
            data: (b'a'..b'z' + 1).map(|c| Frequency { key: c, value: 0 }).collect::<ArrayVec<[_; 26]>>()
        };
        for character in name.bytes().filter(|&x| x != b'-') { freqmap.increment_key(character); }
        freqmap
    }
}

/// Take a character as a byte and wrap add the character by the alphabet. 'a' ... 'z' -> 'a' ... 'z' -> ...
fn wrap_to_char(character: u8, by: u32) -> char {
    ((character as u8 - b'a' + (by % 26) as u8) % 26 + b'a') as char
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
    type Item = (ArrayVec<[char; 5]>, u32);
    fn next(&mut self) -> Option<(ArrayVec<[char; 5]>, u32)> {
        loop {
            if let Some(line) = self.lines.next() {
                let (prefix, checksum) = line.split_at(line.find('[').unwrap());
                let (name, sector_id) = prefix.split_at(line.find(|x: char| x.is_numeric()).unwrap());
                let expected_checksum = checksum[1..checksum.len()-1].as_bytes();

                if &FrequencyMap::from(name).collect_checksum() == expected_checksum {
                    let sector_id = sector_id.parse::<u32>().unwrap();
                    return Some((name.bytes().map(|x| {
                        if x == b'-' { ' ' } else { wrap_to_char(x, sector_id) }
                    }).take(5).collect::<ArrayVec<[_; 5]>>(), sector_id));
                } else {
                    continue
                }
            } else {
                return None;
            }
        }
    }
}

fn room_is_match(room: &[char]) -> bool {
    room == &['n', 'o', 'r', 't', 'h']
}

fn main() {
    let inputs = include_str!("input.txt");
    let mut room_iter = RoomIterator::new(inputs);

    let (mut sum, mut north_room) = (0, 0);
    while let Some(room) = room_iter.next() {
        sum += room.1;
        if room_is_match(&room.0) { north_room = room.1; break }
    }

    for room in room_iter { sum += room.1}

    println!("There sum of valid room sector IDs is {}.", sum);
    println!("The north pole objects are stored in room {}", north_room)
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
