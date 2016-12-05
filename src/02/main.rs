#![feature(alloc_system)]
extern crate alloc_system;

use std::str::Lines;

const PAD: [[char; 3]; 3] = [['1', '2', '3'],
                             ['4', '5', '6'],
                             ['7', '8', '9']];

struct FirstDigitSelector<'a> {
    input: Lines<'a>,
    x: usize,
    y: usize
}

impl<'a> FirstDigitSelector<'a> {
    fn new(input: &'a str) -> FirstDigitSelector<'a> {
        FirstDigitSelector { input: input.lines(), x: 1, y: 1 }
    }
}

impl<'a> Iterator for FirstDigitSelector<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.input.next().map(|line| {
            for character in line.chars() {
                match character {
                    'U' if self.y != 0 => self.y -= 1,
                    'D' if self.y != 2 => self.y += 1,
                    'L' if self.x != 0 => self.x -= 1,
                    'R' if self.x != 2 => self.x += 1,
                    _   => (),
                }
            }
            PAD[self.y][self.x]
        })
    }
}

const PAD_TWO: [[char; 5]; 5] = [['0', '0', '1', '0', '0'],
                                 ['0', '2', '3', '4', '0'],
                                 ['5', '6', '7', '8', '9'],
                                 ['0', 'A', 'B', 'C', '0'],
                                 ['0', '0', 'D', '0', '0']];

struct SecondDigitSelector<'a> {
   input: Lines<'a>,
   x: usize,
   y: usize,
}

impl<'a> SecondDigitSelector<'a> {
   fn new(input: &'a str) -> SecondDigitSelector<'a> {
       SecondDigitSelector { input: input.lines(), x: 0, y: 2 }
   }

   fn can_move_up(&self) -> bool {
       self.x != 0 && self.x != 4 && self.y != 0 && !(self.x == 1 && self.y == 1) && !(self.x == 3 && self.y == 1)
   }

   fn can_move_down(&self) -> bool {
       self.x != 0 && self.x != 4 && self.y != 4 && !(self.x == 1 && self.y == 3) && !(self.x == 3 && self.y == 3)
   }

   fn can_move_left(&self) -> bool {
       self.x != 0 && self.y != 0 && self.y != 4 && !(self.x == 1 && self.y == 1) && !(self.x == 1 && self.y == 3)
   }

   fn can_move_right(&self) -> bool {
       self.x != 4 && self.y != 0 && self.y != 4 && !(self.x == 3 && self.y == 1) && !(self.x == 3 && self.y == 3)
   }
}

impl<'a> Iterator for SecondDigitSelector<'a> {
   type Item = char;
   fn next(&mut self) -> Option<char> {
       self.input.next().map(|line| {
           for character in line.chars() {
               match character {
                   'U' if self.can_move_up()    => self.y -= 1,
                   'D' if self.can_move_down()  => self.y += 1,
                   'L' if self.can_move_left()  => self.x -= 1,
                   'R' if self.can_move_right() => self.x += 1,
                   _   => (),
               }
           }
           PAD_TWO[self.y][self.x]
       })
   }
}

fn main() {
    let inputs = include_str!("input.txt");

    let password = FirstDigitSelector::new(inputs).collect::<String>();
    println!("The password for part one is: {}.", password);

    let password = SecondDigitSelector::new(inputs).collect::<String>();
    println!("The password for part two is: {}.", password);
}

#[test]
fn pad_one_test() {
    let input = "ULL\nRRDDD\nLURDL\nUUUUD\n";
    let expected = vec!['1', '9', '8', '5'];
    for (actual, expected) in FirstDigitSelector::new(input).zip(expected.into_iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn pad_two_test() {
    let input = "ULL\nRRDDD\nLURDL\nUUUUD\n";
    let expected = vec!['5', 'D', 'B', '3'];
    for (actual, expected) in SecondDigitSelector::new(input).zip(expected.into_iter()) {
        assert_eq!(actual, expected);
    }
}
