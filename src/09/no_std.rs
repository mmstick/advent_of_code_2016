#![no_std]
#![feature(libc)]
#![feature(start)]
#![feature(alloc_system)]
extern crate alloc_system;
extern crate time;
extern crate libc;

extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const INPUT: &'static str = include_str!("input.txt");

/// The base structure of the `Decompressor` `Iterator` contains the data as a
/// string slice and keeps track of the current state via the `read` field.
struct Decompressor<'a> {
    data: &'a str,
    read: usize,
}

impl<'a> Decompressor<'a> {
    fn new(data: &'a str) -> Decompressor<'a> {
        Decompressor { data: data, read: 0 }
    }
}

/// A `DecompressionToken` may either be a regular string slice that does not
/// need to be further searched, or a marker that contains the repeat count
/// and the string slice to repeat, which may or may not contain even more
/// `DecompressionToken`s.
#[derive(PartialEq, Debug)]
enum DecompressionToken<'a> {
    Regular(&'a str),
    Marker(u8, &'a str),
}

impl<'a> Iterator for Decompressor<'a> {
    type Item = DecompressionToken<'a>;
    fn next(&mut self) -> Option<DecompressionToken<'a>> {
        // Determines if the next token to return will be a `Regular` or `Marker` token.
        let found_marker = if let Some(character) = self.data.chars().nth(self.read) {
            self.read += 1;
            character == '('
        } else {
            return None
        };

        // Chooses the correct loop to execute accordingly.
        if found_marker {
            let mut start = self.read;
            let mut charas = 0usize;
            let mut found_first_num = false;

            // First find the number of characters that will be repeated before
            // the first 'x', and then find the number of times to repeat
            // before the ')' character that follows.
            for character in self.data.chars().skip(self.read) {
                if found_first_num {
                    if character == ')' {
                        let repeat = self.data[start..self.read].parse::<u8>().unwrap();
                        start = self.read + 1;
                        self.read += charas + 1;
                        return Some(DecompressionToken::Marker(repeat, &self.data[start..start + charas]));
                    }
                } else if character == 'x' {
                    charas = self.data[start..self.read].parse::<usize>().unwrap();
                    found_first_num = true;
                    start = self.read + 1;
                }
                self.read += 1;
            }
            None // Error
        } else {
            let start = self.read - 1;

            // A `Regular` token ends when either a '(' is found or the
            // internal data has been exhausted.
            for character in self.data.chars().skip(self.read) {
                if character == '(' {
                    return Some(DecompressionToken::Regular(&self.data[start..self.read]));
                }
                self.read += 1;
            }
            Some(DecompressionToken::Regular(&self.data[start..]))
        }
    }
}

/// Calculates the size of the file if it was using version one of the format.
fn calculate_size_p1(input: &str) -> usize {
    use DecompressionToken::{Marker, Regular};
    let mut decompressed_length = 0;
    for token in Decompressor::new(input) {
        match token {
            Marker(repeat, string) => decompressed_length += string.len() * repeat as usize,
            Regular(string)        => decompressed_length += string.len(),
        }
    }
    decompressed_length
}

/// Calculates the actual version two file size of the decompressed file.
fn calculate_size_p2(input: &str) -> usize {
    use DecompressionToken::{Marker, Regular};
    let mut decompressed_length = 0;
    for token in Decompressor::new(input) {
        match token {
            Marker(repeat, string) => decompressed_length += calculate_size_p2(string) * repeat as usize,
            Regular(string)        => decompressed_length += string.len(),
        }
    }
    decompressed_length
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let begin = time::precise_time_ns();
    let decompressed_length_p1 = calculate_size_p1(INPUT);
    let decompressed_length_p2 = calculate_size_p2(INPUT);
    let end = time::precise_time_ns();
    unsafe {
        printf(b"The decompressed length of version one is %d bytes (%d KiB).\n\0".as_ptr(),
            decompressed_length_p1, decompressed_length_p1 / 1024);
        printf(b"The decompressed length of version two is %lld bytes (%d GiB)\n\0".as_ptr(),
            decompressed_length_p2, decompressed_length_p2 / 1024 / 1024 / 1024);
        printf(b"Day 09: Execution Time: %f milliseconds\n\0".as_ptr(),
            ((end - begin) as f64) / 1_000_000f64);
    }
    0
}

#[test]
fn decompressor_test() {
    let input = "ADVENTA(1x5)BC(3x3)XYZA(2x2)BCD(2x2)EFG(6x1)(1x3)AX(8x2)(3x3)ABCY";
    let expected = [
        DecompressionToken::Regular("ADVENTA"),
        DecompressionToken::Marker(5, "B"),
        DecompressionToken::Regular("C"),
        DecompressionToken::Marker(3, "XYZ"),
        DecompressionToken::Regular("A"),
        DecompressionToken::Marker(2, "BC"),
        DecompressionToken::Regular("D"),
        DecompressionToken::Marker(2, "EF"),
        DecompressionToken::Regular("G"),
        DecompressionToken::Marker(1, "(1x3)A"),
        DecompressionToken::Regular("X"),
        DecompressionToken::Marker(2, "(3x3)ABC"),
        DecompressionToken::Regular("Y")
    ];

    for (actual, expected) in Decompressor::new(input).zip(expected.iter()) {
        assert_eq!(actual, *expected);
    }

    assert_eq!(Decompressor::new(input).count(), 13);
}