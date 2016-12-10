#![feature(step_by)]
#![feature(alloc_system)]
extern crate alloc_system;
extern crate arrayvec;
extern crate time;

use arrayvec::ArrayVec;
use std::io::{self, Write};
use std::{u8, u64};

const INPUT:  &'static str = include_str!("input.txt");
const MASK_6:           u8 = 0b111111;
const MASK_50:         u64 = 0b11111111111111111111111111111111111111111111111111;


fn mersenne_generator(num: u64) -> u64 {
    (0..num).fold(0, |acc, _| acc + acc + 1)
}

trait Screen {
    fn enabled_pixels(&self) -> u32;
    fn rect(&mut self, wide: u8, tall: u8);
    fn rotate_row(&mut self, row: u8, shift: u8);
    fn rotate_column(&mut self, column: u8, shift: u8);
    fn display_pixels(&self);
}

struct Screen8 {
    data: [u8; 50]
}

impl Default for Screen8 {
    fn default() -> Screen8 { Screen8 { data: [0; 50] } }
}

impl Screen for Screen8 {
    fn enabled_pixels(&self) -> u32 {
        self.data.iter().fold(0, |acc, x| acc + x.count_ones())
    }

    fn rect(&mut self, wide: u8, tall: u8) {
        let bytes_enable = mersenne_generator(tall as u64) as u8;
        for row in 0..wide { self.data[row as usize] |= bytes_enable; }
    }

    fn rotate_column(&mut self, column: u8, shift: u8) {
        let mask_bits = mersenne_generator((6 - shift) as u64) as u8;
        let mut current_value = self.data[column as usize];
        let high = current_value & (u8::MAX ^ mask_bits);
        self.data[column as usize] = if high == 0 {
            current_value << shift
        } else {
            current_value <<= shift;
            current_value |= high >> (6 - shift);
            current_value & MASK_6
        };
    }

    fn rotate_row(&mut self, row: u8, shift: u8) {
        let row_mask     = 1 << row;
        let mut new_data = [0; 50];
        for (id, row) in self.data.iter().enumerate() {
            new_data[id] = row & (u8::MAX ^ row_mask);
        }

        for (mut id, row) in self.data.iter().enumerate() {
            id += shift as usize;
            new_data[if id > 49 { id - 50 } else { id }] += row & row_mask;
        }

        for (id, row) in self.data.iter_mut().enumerate() {
            *row = new_data[id];
        }
    }

    fn display_pixels(&self) {
        let stdout     = io::stdout();
        let mut stdout = stdout.lock();
        for column in 0..6 {
            let row_mask = 1 << column;
            for row in 0..49 {
                if self.data[row] & row_mask == 0 {
                    stdout.write(b" ").unwrap();
                } else {
                    stdout.write(b"#").unwrap();
                }
            }
            stdout.write(b"\n").unwrap();
        }
    }
}

struct Screen64 {
    data: [u64; 6]
}

impl Default for Screen64 {
    fn default() -> Screen64 { Screen64 { data: [0; 6] } }
}

impl Screen for Screen64 {
    fn enabled_pixels(&self) -> u32 {
        self.data.iter().fold(0, |acc, x| acc + x.count_ones())
    }

    fn rect(&mut self, wide: u8, tall: u8) {
        let bytes_enable = mersenne_generator(wide as u64);
        for column in 0..tall { self.data[column as usize] |= bytes_enable; }
    }

    fn rotate_row(&mut self, row: u8, shift: u8) {
        let mask_bits = mersenne_generator((50 - shift) as u64);
        let mut current_value = self.data[row as usize];
        let high = current_value & (u64::MAX ^ mask_bits);
        self.data[row as usize] = if high == 0 {
            current_value << shift
        } else {
            current_value <<= shift;
            current_value |= high >> (50 - shift);
            current_value & MASK_50
        };
    }

    fn rotate_column(&mut self, column: u8, shift: u8) {
        let column_mask  = 1 << column;
        let mut new_data = [0; 6];

        for (id, column) in self.data.iter().enumerate() {
            new_data[id] = column & (u64::MAX ^ column_mask);
        }

        for (id, column) in self.data.iter().enumerate() {
            new_data[id] = column & (u64::MAX ^ column_mask);
        }

        for (mut id, column) in self.data.iter().enumerate() {
            id += shift as usize;
            new_data[if id > 5 { id - 6 } else { id }] += column & column_mask;
        }

        for (id, column) in self.data.iter_mut().enumerate() {
            *column = new_data[id];
        }
    }

    fn display_pixels(&self) {
        let stdout     = io::stdout();
        let mut stdout = stdout.lock();
        for row in 0..6 {
            for column in 0..49 {
                let column_mask = 1 << column;
                if self.data[row] & column_mask == 0 {
                    stdout.write(b" ").unwrap();
                } else {
                    stdout.write(b"#").unwrap();
                }
            }
            stdout.write(b"\n").unwrap();
        }
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Rect(u8, u8),
    RotateRow(u8, u8),
    RotateColumn(u8, u8),
    Error
}

fn convert_to_action(input: &str) -> Action {
    let elements = input.split_whitespace().collect::<ArrayVec<[&str; 5]>>();
    match elements[0] {
        "rect" => {
            let values = elements[1].split('x').collect::<ArrayVec<[&str; 2]>>();
            Action::Rect(values[0].parse::<u8>().unwrap(), values[1].parse::<u8>().unwrap())
        },
        "rotate" => {
            let first_values = elements[2].split('=').collect::<ArrayVec<[&str; 2]>>();
            let x = first_values[1].parse::<u8>().unwrap();
            let y = elements[4].parse::<u8>().unwrap();
            if elements[1] == "row" {
                Action::RotateRow(x, y)
            } else {
                Action::RotateColumn(x, y)
            }
        },
        _ => Action::Error
    }
}

fn lit_pixels<S: Screen>(input: &str, mut screen: S) -> u32 {
    for line in input.lines() {
        match convert_to_action(line) {
            Action::Rect(wide, tall)            => screen.rect(wide, tall),
            Action::RotateColumn(column, shift) => screen.rotate_column(column, shift),
            Action::RotateRow(row, shift)       => screen.rotate_row(row, shift),
            _ => unreachable!() // hopefully
        }
    }
    screen.display_pixels();
    screen.enabled_pixels()
}

fn main() {
    let begin = time::precise_time_ns();
    let enabled_pixels = lit_pixels(INPUT, Screen64::default());
    let end = time::precise_time_ns();
    println!("There are {} enabled pixels.", enabled_pixels);
    println!("Day 08 Execution Time: {} milliseconds", ((end - begin) as f64) / 1_000_000f64);
}

#[test]
fn test_rect() {
    let mut screen = Screen8::default();
    screen.rect(1, 1);
    assert_eq!(screen.data[0], 0b1);
    assert_eq!(screen.data[1], 0);
    assert_eq!(screen.enabled_pixels(), 1);
    screen.rect(1, 3);
    assert_eq!(screen.data[0], 0b111);
    assert_eq!(screen.data[1], 0);
    assert_eq!(screen.enabled_pixels(), 3);
    screen.rect(2, 2);
    assert_eq!(screen.data[0], 0b111);
    assert_eq!(screen.data[1], 0b11);
    assert_eq!(screen.enabled_pixels(), 5);
    screen.rect(3, 6);
    assert_eq!(screen.data[0], 0b111111);
    assert_eq!(screen.data[1], 0b111111);
    assert_eq!(screen.data[2], 0b111111);
    assert_eq!(screen.enabled_pixels(), 18);

    let mut screen = Screen64::default();
    screen.rect(3, 6);
    for row in 0..6 { assert_eq!(screen.data[row], 0b111); }
}

#[test]
fn test_rows() {
    let mut screen = Screen8::default();
    screen.rect(50, 1);
    assert_eq!(screen.enabled_pixels(), 50);
    for column in (0..50).step_by(2) { screen.rotate_column(column, 1); }
    assert_eq!(screen.enabled_pixels(), 50);
    assert_eq!(screen.data[0], 0b10);
    assert_eq!(screen.data[1], 0b1);
    screen.rotate_row(0, 1);
    assert_eq!(screen.data[0], 0b11);
    assert_eq!(screen.data[1], 0);
    assert_eq!(screen.enabled_pixels(), 50);
    screen.rect(50, 1);
    assert_eq!(screen.data[0], 0b11);
    assert_eq!(screen.data[1], 0b1);
    assert_eq!(screen.enabled_pixels(), 75);
    screen.rotate_column(0, 3);
    screen.rotate_column(1, 3);
    assert_eq!(screen.data[0], 0b11000);
    assert_eq!(screen.data[1], 0b1000);
}

#[test]
fn test_columns() {
    let mut screen = Screen8::default();
    screen.rect(30, 3);
    assert_eq!(screen.data[0], 0b111);
    assert_eq!(screen.data[30], 0);
    assert_eq!(screen.enabled_pixels(), 90);
    screen.rotate_column(0, 3);
    assert_eq!(screen.data[0], 0b111000);
    screen.rotate_column(0, 4); 
    assert_eq!(screen.data[0], 0b001110);
    screen.rotate_column(0, 3);
    assert_eq!(screen.data[0], 0b110001); 
    assert_eq!(screen.enabled_pixels(), 90);

    let mut screen = Screen64::default();
    screen.rect(3, 3);
    for column in 0..3 { assert_eq!(screen.data[column], 7); }
    screen.rotate_column(0, 3);
    assert_eq!(screen.data[3], 1);
}

#[test]
fn convert() {
    let input = r#"rect 1x1
    rotate row y=0 by 6
    rect 1x1
    rotate row y=0 by 3
    rect 1x1
    rotate row y=0 by 5
    rect 1x1
    rotate row y=0 by 4
    rect 2x1"#;

    let expected = [Action::Rect(1, 1), Action::RotateRow(0, 6), Action::Rect(1, 1),
        Action::RotateRow(0, 3), Action::Rect(1, 1), Action::RotateRow(0, 5),
        Action::Rect(1, 1), Action::RotateRow(0, 4), Action::Rect(2, 1)];

    for (actual, expected) in input.lines().map(convert_to_action).zip(expected.iter()) {
        assert_eq!(actual, *expected);
    }
}

#[test]
fn test_input() {
    let number_of_actions = INPUT.lines().count();
    let actual = INPUT.lines().map(convert_to_action).filter(|x| match *x {
        Action::Error => false,
        _ => true,
    }).count();
    assert_eq!(number_of_actions, actual);

    let input = r#"rect 1x1
    rotate row y=0 by 6
    rect 1x1
    rotate row y=0 by 3
    rect 1x1
    rotate row y=0 by 5
    rect 1x1
    rotate row y=0 by 4
    rect 2x1"#;

    assert_eq!(6, lit_pixels(input, Screen8::default()));
    assert_eq!(6, lit_pixels(input, Screen64::default()));
    assert_eq!(116, lit_pixels(INPUT, Screen8::default()));
    assert_eq!(116, lit_pixels(INPUT, Screen64::default()));
}

#[test]
fn mersenne_test() {
    assert_eq!(7, mersenne_generator(3));
}
