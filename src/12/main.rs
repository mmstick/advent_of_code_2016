#![feature(alloc_system)]
extern crate alloc_system;
extern crate arrayvec;
extern crate time;

use arrayvec::ArrayVec;
use std::iter::Enumerate;
use std::str::Lines;

const INPUT: &'static str = include_str!("input.txt");

struct InstructionIter<'a> {
    instructions: Enumerate<Lines<'a>>,
}

impl<'a> InstructionIter<'a> {
    fn new(input: &'a str) -> InstructionIter<'a> {
        InstructionIter { instructions: input.lines().enumerate() }
    }
}

#[derive(Debug)]
enum Instruction {
    /// Optimized instruction to add x to y and set x to 0.
    Add(usize, usize),
    /// Copy value `i64` into register `usize`
    CopyInteger(i64, usize),
    /// Copy from left register into right register 
    CopyRegister(usize, usize),
    /// Increment register
    Increment(usize),
    /// Decrement register
    Decrement(usize),
    /// Skip by `i64` if register or value is not zero
    JumpIf(Kind, i64),
    /// Do nothing, successfully
    NoOp,
}

#[derive(Debug)]
enum Kind {
    Register(usize),
    Value(i64),
}

enum InstructionErr<'a> {
    /// Attempted to copy a register into a value
    CopyRegisterToValue(usize),
    /// Jump value is not valid
    JumpInvalid(usize),
    /// No register for instruction `&'a str` at line `usize`
    NoRegister(&'a str, usize),
    /// No register or value supplied for the Copy instruction.
    NoRegisterOrValue(usize),
    /// Invalid instruction `&'a str` at line `usize`
    InvalidInstruction(&'a str, usize),
}

fn char_to_index(input: char) -> usize { ((input as u8) - 97) as usize }

impl<'a> Iterator for InstructionIter<'a> {
    type Item = Result<Instruction, InstructionErr<'a>>;
    fn next(&mut self) -> Option<Result<Instruction, InstructionErr<'a>>> {
        let (line, mut element) = match self.instructions.next() {
            Some((line, instruction)) => (line, instruction.split_whitespace()),
            None       => return None
        };
        let instruction = element.next().unwrap();
        match instruction {
            "cpy" => {
                let first = match element.next() {
                    Some(element) => element,
                    None => return Some(Err(InstructionErr::NoRegisterOrValue(line)))
                };

                let second = match element.next() {
                    Some(element) => element,
                    None => return Some(Err(InstructionErr::NoRegisterOrValue(line)))
                };

                match first.parse::<i64>() {
                    Ok(value) => {
                        let register = second.chars().next().unwrap();
                        return if register.is_numeric() {
                            Some(Err(InstructionErr::NoRegister(instruction, line)))
                        } else {
                            Some(Ok(Instruction::CopyInteger(value, char_to_index(register))))
                        }
                    }
                    Err(_) => {
                        let first = first.chars().next().unwrap();
                        let second = second.chars().next().unwrap();
                        return if first.is_numeric() || second.is_numeric() {
                            Some(Err(InstructionErr::CopyRegisterToValue(line)))
                        } else {
                            Some(Ok(Instruction::CopyRegister(char_to_index(first), char_to_index(second))))
                        }
                    }
                }
            },
            "inc" => {
                let register = match element.next() {
                    Some(register) => char_to_index(register.chars().next().unwrap()),
                    None => return Some(Err(InstructionErr::NoRegister(instruction, line)))
                };
                
                return Some(Ok(Instruction::Increment(register)))
            }, 
            "dec" => {
                let register = match element.next() {
                    Some(register) => char_to_index(register.chars().next().unwrap()),
                    None => return Some(Err(InstructionErr::NoRegister(instruction, line)))
                };

                return Some(Ok(Instruction::Decrement(register)))
            },
            "jnz" => {
                let kind = match element.next() {
                    Some(kind) => match kind.parse::<i64>() {
                        Ok(value) => Kind::Value(value),
                        Err(_)    => Kind::Register(char_to_index(kind.chars().next().unwrap())),
                    },
                    None => return Some(Err(InstructionErr::NoRegister(instruction, line)))
                };

                let back = match element.next().map(|x| x.parse::<i64>()) {
                    Some(Ok(back)) => back,
                    _ => return Some(Err(InstructionErr::JumpInvalid(line)))
                };

                return Some(Ok(Instruction::JumpIf(kind, back)))
            },
            _ => return Some(Err(InstructionErr::InvalidInstruction(instruction, line)))
        }
    }
}

fn instruction_optimizer(input: &str) -> ArrayVec<[Instruction; 32]> {
    let mut output: ArrayVec<[Instruction; 32]> = ArrayVec::new();
    let mut matched = 0;
    let (mut inc_reg, mut dec_reg) = (0, 0);
    for instruction in InstructionIter::new(input) {
        match instruction {
            Ok(instruction) => match instruction {
                Instruction::Increment(register) if matched == 0 => {
                    inc_reg = register;
                    matched = 1;
                },
                Instruction::Decrement(register) if matched == 1 => {
                    dec_reg = register;
                    matched = 2;
                },
                Instruction::JumpIf(Kind::Register(register), value) if matched == 2 => {
                    if register == dec_reg && value == -2 {
                        output.push(Instruction::NoOp);
                        output.push(Instruction::NoOp);
                        output.push(Instruction::Add(dec_reg, inc_reg));
                    } else {
                        output.push(Instruction::Increment(inc_reg));
                        output.push(Instruction::Decrement(dec_reg));
                        output.push(instruction);
                    }
                    inc_reg = 0;
                    dec_reg = 0;
                    matched = 0;
                }
                _ => {
                    if matched == 1 {
                        output.push(Instruction::Increment(inc_reg));
                        inc_reg = 0;
                        matched = 0;
                    } else if matched == 2 {
                        output.push(Instruction::Increment(inc_reg));
                        output.push(Instruction::Decrement(dec_reg));
                        inc_reg = 0;
                        dec_reg = 0;
                        matched = 0;
                    }
                    output.push(instruction);
                }
            },
            Err(error) => match error {
                InstructionErr::CopyRegisterToValue(line) => {
                    panic!("11: attempted to copy a register into a value at line {}.", line);
                },
                InstructionErr::InvalidInstruction(instruction, line) => {
                    panic!("11: invalid instruction, '{}', at line {}.", instruction, line);
                },
                InstructionErr::JumpInvalid(line) => {
                    panic!("11: jump value is invalid at line {}.", line);
                },
                InstructionErr::NoRegister(instruction, line) => {
                    panic!("11: no register for instruction '{}' at line {}.", instruction, line);
                },
                InstructionErr::NoRegisterOrValue(line) => {
                    panic!("11: no value or register for cpy was supplied at line {}.", line);
                }
            }
        }
    }

    output
}

fn jump(read: usize, value: i64) -> usize {
    if value < 0 {
        read - (value.abs() as usize + 1)
    } else {
        read + value as usize - 1
    }
}

fn calculate(registers: &mut [i64], input: &str) -> i64 {
    let instructions = instruction_optimizer(input);
    let mut index = 0;
    let max = instructions.len();
    while index < max {
        match instructions[index] {
            Instruction::Add(x, y)                      => { registers[y] += registers[x]; registers[x] = 0; },
            Instruction::NoOp                           => (),
            Instruction::CopyInteger(integer, register) => registers[register]  = integer,
            Instruction::CopyRegister(x, y)             => registers[y]         = registers[x],
            Instruction::Decrement(register)            => registers[register] -= 1,
            Instruction::Increment(register)            => registers[register] += 1,
            Instruction::JumpIf(ref kind, step) => match *kind {
                Kind::Register(register) => if registers[register] != 0 { index = jump(index, step); },
                Kind::Value(value)       => if value != 0 { index = jump(index, step); }
            }
        }
        index += 1
    }
    registers[0]
}

fn main() {
    let begin = time::precise_time_ns();
    let mut registers_one = [0i64; 4];
    let mut registers_two = [0i64, 0, 1, 0];
    let one = calculate(&mut registers_one, INPUT);
    let two = calculate(&mut registers_two, INPUT);
    let end = time::precise_time_ns();
    println!("The value of register a in part one is {}.", one);
    println!("The value of register a in part two is {}.", two);
    println!("Day 12 Execution Time: {} milliseconds", (end - begin) as f64 / 1_000_000f64)
}

#[test]
fn part_one() {
    let input = r#"cpy 41 a
        inc a
        inc a
        dec a
        jnz a 2
        dec a"#;
    assert_eq!(42, value_of_a(input));
}