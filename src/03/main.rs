#![feature(alloc_system)]
extern crate alloc_system;

use std::cmp::max;
use std::str::Lines;

/// Given the size of each side, this determines if the triangle is possible
fn is_possible(x: u16, y: u16, z: u16) -> bool {
    let sum = x + y + z;
    let max = max(max(x, y), z);
    sum - max > max
}

/// Iterates one row at a time and returns `true` if the row is a possible triangle.
struct TriangleRowIterator<'a> { triangles: Lines<'a> }

impl<'a> TriangleRowIterator<'a> {
    fn new(input: &'a str) -> TriangleRowIterator<'a> { TriangleRowIterator { triangles: input.lines() } }
}

impl<'a> Iterator for TriangleRowIterator<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        self.triangles.next().map(|line| {
            let mut sides = line.split_whitespace().map(|x| x.parse::<u16>().unwrap());
            is_possible(sides.next().unwrap(), sides.next().unwrap(), sides.next().unwrap())
        })
    }
}

/// Iterates three rows at a time and returns the number of triangles that are were possible in each iteration.
struct TriangleColumnIterator<'a> { rows: Lines<'a> }

impl<'a> TriangleColumnIterator<'a> {
    fn new(input: &'a str) -> TriangleColumnIterator<'a> { TriangleColumnIterator { rows: input.lines() } }
}

impl<'a> Iterator for TriangleColumnIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let first_row  = self.rows.next().map(|x| x.split_whitespace().map(|x| x.parse::<u16>().unwrap()));
        let second_row = self.rows.next().map(|x| x.split_whitespace().map(|x| x.parse::<u16>().unwrap()));
        let third_row  = self.rows.next().map(|x| x.split_whitespace().map(|x| x.parse::<u16>().unwrap()));
        if let (Some(first), Some(second), Some(third)) = (first_row, second_row, third_row) {
            Some(first.zip(second).zip(third).filter(|&((x,y),z)| is_possible(x,y,z)).count())
        } else {
            None
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let valid_triangles = TriangleRowIterator::new(input).filter(|&x| x).count();
    println!("There are {} valid row-based triangles.", valid_triangles);

    let valid_triangles = TriangleColumnIterator::new(input).fold(0, |acc, next| acc + next);
    println!("There are {} valid column-based triangles.", valid_triangles);
}

#[test]
fn part_one() {
    let input = include_str!("input.txt");
    let valid_triangles = TriangleRowIterator::new(input).filter(|&x| x).count();
    assert_eq!(982, valid_triangles);
}

#[test]
fn part_two() {
    let input = include_str!("input.txt");
    let valid_triangles = TriangleColumnIterator::new(input).fold(0, |acc, next| acc + next);
    assert_eq!(1826, valid_triangles);
}
