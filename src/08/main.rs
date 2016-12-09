#![feature(alloc_system)]
extern crate alloc_system;
extern crate arrayvec;
extern crate time;

use arrayvec::ArrayVec;
use std::io::{self, Write};

const INPUT:             &'static str = include_str!("input.txt");
const BYTE_LOOKUP:   &'static [u8; 6] = &[1, 3, 7, 15, 31, 63];
const ROTATE_LOOKUP: &'static [u8; 5] = &[31, 15, 7, 3, 1];
const ROW_MASKS:     &'static [u8; 6] = &[0b1, 0b10, 0b100, 0b1000, 0b10000, 0b100000];

struct Screen {
    data: [u8; 50]
}

impl Default for Screen {
    fn default() -> Screen { Screen { data: [0; 50]} }
}

impl Screen {
    fn enabled_pixels(&self) -> u32 {
        self.data.iter().fold(0, |acc, x| acc + x.count_ones())
    }

    fn rect(&mut self, wide: u8, tall: u8) {
        let bytes_enable = if tall > 0 && tall < 7 {
            BYTE_LOOKUP[(tall - 1) as usize]
        } else {
            return
        };

        for row in 0..wide { self.data[row as usize] |= bytes_enable; }
    }

    fn rotate_column(&mut self, column: u8, shift: u8) {
        let mask_bits = if shift > 0 && shift < 6 {
            ROTATE_LOOKUP[(shift - 1) as usize]
        } else {
            return
        };

        let mut current_value = self.data[column as usize];
        let high = current_value & (255 ^ mask_bits);
        self.data[column as usize] = if high == 0 {
            current_value << shift
        } else {
            current_value = current_value << shift;
            current_value |= high >> (6 - shift);
            current_value & (255 ^ (64 + 128))
        };
    }

    fn rotate_row(&mut self, row: u8, shift: u8) {
        let row_mask     = ROW_MASKS[row as usize];
        let mut new_data = [0; 50];
        for (id, row) in self.data.iter().enumerate() {
            new_data[id] = row & (255 ^ row_mask);
        }

        for (mut id, row) in self.data.iter().enumerate() {
            id = id + shift as usize;
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
            let row_mask = ROW_MASKS[column];
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

fn lit_pixels(input: &str) -> u32 {
    let mut screen = Screen::default();
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
    let enabled_pixels = lit_pixels(INPUT);
    let end = time::precise_time_ns();
    println!("There are {} enabled pixels.", enabled_pixels);
    println!("Day 08 Execution Time: {} milliseconds", ((end - begin) as f64) / 1_000_000f64);
}

#[test]
fn test_rect() {
    let mut screen = Screen::default();
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
}

#[test]
fn test_rows() {
    let mut screen = Screen::default();
    screen.rect(50, 1);
    assert_eq!(screen.enabled_pixels(), 50);
    for column in (0..50).step_by(2) {
        screen.rotate_column(column, 1);
    }
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

    let mut screen = Screen::default();
    screen.rect(1, 1);
    assert_eq!(screen.enabled_pixels(), 1);
    screen.rotate_row(0, 6);
    screen.rect(1, 1);
    println!("{:b}", screen.data[0]);
    println!("{:b}", screen.data[6]);
    assert_eq!(screen.enabled_pixels(), 2);
}

#[test]
fn test_columns() {
    let mut screen = Screen::default();
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

    assert_eq!(6, lit_pixels(input));
    assert_eq!(116, lit_pixels(INPUT));
}
