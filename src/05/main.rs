#![feature(alloc_system)]
extern crate alloc_system;
extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

const PREFIX: &'static str = "wtnhxymk";
const PREFIX_LEN:    usize = 8;
const MASK_SECOND_NIBBLE: u8 = 255 ^ (16 + 32 + 64 + 128);

/// Converts a hexadecimal number ranging from 0 to 15 into a character.
fn to_char(num: u8) -> char { if num < 10 { (num + 48) as char } else { (num + 87) as char } }

/// Simultaneously checks the first five nibbles of three bytes for zeroness.
fn contains_five_zeroes(x: u8, y: u8, z: u8) -> bool { x | y | (z >> 4) == 0 }

/// Iterates seemingly-endlessly, checking for hashes whose first five nibbles are zero and returning the
/// sixth and seventh characters as a single `u8` byte when that condition is true.
struct DoorHasher {
    index:  u32,
    hash:   String,
    digest: [u8; 16],
    sh:     Md5,
}

impl DoorHasher {
    fn new() -> DoorHasher { DoorHasher { index: 0, hash: String::from(PREFIX), digest: [0u8; 16], sh: Md5::new() } }
}

impl Iterator for DoorHasher {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        loop {
            self.sh.reset();
            self.hash.truncate(PREFIX_LEN);
            self.hash.push_str(&self.index.to_string());
            self.index += 1;
            self.sh.input_str(&self.hash);
            self.sh.result(&mut self.digest);
            if contains_five_zeroes(self.digest[0], self.digest[1], self.digest[2]) {
                let first_nibble = self.digest[2] & MASK_SECOND_NIBBLE;
                let second_nibble = self.digest[3] >> 4;
                return Some(second_nibble | first_nibble << 4)
            } else {
                continue
            }
        }
    }
}

/// Collects the passwords for both the first and second door simultaneously
fn collect_passwords(first_password: &mut [char; 8], second_password: &mut [char; 8]) {
    let (mut first_matched, mut second_matched) = (0, 0);
    for character_pair in DoorHasher::new() {
        let sixth_char = character_pair >> 4;
        if first_matched < 8 {
            first_password[first_matched] = to_char(sixth_char);
            first_matched += 1;
        }
        if sixth_char < 8 && second_password[sixth_char as usize] == '\0' {
            second_password[sixth_char as usize] = to_char(character_pair & MASK_SECOND_NIBBLE);
            second_matched += 1;
            if second_matched == 8 { break }
        }
    }
}

fn main() {
    let mut first_password:  [char; 8] = ['\0'; 8];
    let mut second_password: [char; 8] = ['\0'; 8];
    collect_passwords(&mut first_password, &mut second_password);

    println!("The first door's password is {}.\nThe second door's password is {}.",
        first_password.iter().cloned().collect::<String>(),
        second_password.iter().cloned().collect::<String>());
}

#[test]
fn test_hash() {
    let mut first_password:  [char; 8] = ['\0'; 8];
    let mut second_password: [char; 8] = ['\0'; 8];
    collect_passwords(&mut first_password, &mut second_password);
    assert_eq!(String::from("2414bc77"), first_password.iter().cloned().collect::<String>());
    assert_eq!(String::from("437e60fc"), second_password.iter().cloned().collect::<String>());
}

#[test]
fn test_nums_to_chars() {
    let expected = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    for (actual, expected) in (0..16).map(to_char).zip(expected.iter()) {
        assert_eq!(actual, *expected);
    }
}
