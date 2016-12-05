#![feature(alloc_system)]
extern crate alloc_system;

// Set `Direction` as a `Copy` type because it is an 8-bit value, making it more expensive to reference.
#[derive(Copy, Clone)]
enum Direction { West, North, East, South }

#[derive(PartialEq)]
enum Angle { Left, Right }

/// Takes an `Angle` as input against the current `Direction` and uses that to determine the new `Direction`.
fn change_direction(current: Direction, angle: Angle) -> Direction {
    match current {
        Direction::West  => if angle == Angle::Left { Direction::South } else { Direction::North },
        Direction::North => if angle == Angle::Left { Direction::West  } else { Direction::East },
        Direction::East  => if angle == Angle::Left { Direction::North } else { Direction::South },
        Direction::South => if angle == Angle::Left { Direction::East  } else { Direction::West },
    }
}

/// Updates the current position based on the given `Direction` and the amount of blocks to move forward.
fn move_position(position: &mut (isize, isize), direction: Direction, forward: isize) {
    match direction {
        Direction::West  => position.0 -= forward,
        Direction::North => position.1 += forward,
        Direction::East  => position.0 += forward,
        Direction::South => position.1 -= forward,
    }
}

/// Determines the angle based on the character being either "L" or "R"
fn angle_from(input: &str) -> Angle { if input == "L" { Angle::Left } else { Angle::Right } }

/// Parses the number of blocks to move forward
fn blocks_forward(input: &str) -> isize { input.parse::<isize>().unwrap() }

/// Calculates the distance of the final point and returns it's position for the first part of the puzzle.
fn calculate_distance_for_final_point(inputs: &str) -> ((isize, isize), isize) {
    let mut position = (0isize, 0isize);
    let mut direction = Direction::North;

    for input in inputs.split(", ") {
        direction = change_direction(direction, angle_from(&input[0..1]));
        move_position(&mut position, direction, blocks_forward(&input[1..]));
    }

    (position, position.0.abs() + position.1.abs())
}

/// Keeps track of what blocks have been visited before, and returning the first collision if it is detected.
fn check_collision(visited: &mut Vec<(isize, isize)>, current: &mut (isize, isize), direction: Direction,
    forward: isize) -> Option<(isize, isize)>
{
    macro_rules! increment {
        ($increment:stmt) => {{
            for _ in 0..forward {
                $increment;
                if visited.contains(current) { return Some(*current); } else { visited.push(*current); }
            }
        }}
    }

    match direction {
        Direction::West  => increment!(current.0 -= 1),
        Direction::North => increment!(current.1 += 1),
        Direction::East  => increment!(current.0 += 1),
        Direction::South => increment!(current.1 -= 1)
    }

    None
}

/// Calculates the distance of the first block that is visited twice and returns it's position for the second part.
fn calculate_distance_from_hq(inputs: &str) -> ((isize, isize), isize) {
    let mut position = (0isize, 0isize);
    let mut direction = Direction::North;
    let mut visited = Vec::with_capacity(248);
    visited.push(position);

    for input in inputs.split(", ") {
        direction = change_direction(direction, angle_from(&input[0..1]));
        if let Some(collision) = check_collision(&mut visited, &mut position, direction, blocks_forward(&input[1..])) {
            return (collision, collision.0.abs() + collision.1.abs());
        }
    }

    (position, position.0.abs() + position.1.abs())
}

fn main() {
    let inputs = include_str!("input.txt");

    let (position, distance) = calculate_distance_for_final_point(&inputs[0..inputs.len()-1]);
    println!("The last point is at ({}, {}), which is {} blocks away.", position.0, position.1, distance);

    let (position, distance) = calculate_distance_from_hq(&inputs[0..inputs.len()-1]);
    println!("The Easter Bunny HQ is at ({}, {}), which is {} blocks away.", position.0, position.1, distance);
}

#[test]
fn part_one() {
    let inputs = "R5, L5, R5, R3";
    assert_eq!(((10,2), 12), calculate_distance_for_final_point(inputs));
    let inputs = "R2, R2, R2";
    assert_eq!(((0,-2), 2), calculate_distance_for_final_point(inputs));
    let inputs = "R2, L3";
    assert_eq!(((2,3), 5), calculate_distance_for_final_point(inputs));
}

#[test]
fn part_two() {
    let inputs = "R8, R4, R4, R8";
    assert_eq!(((4,0), 4), calculate_distance_from_hq(inputs));
    let inputs = "R4, L1, R1, L2, L2, L2, R2, L2";
    assert_eq!(((1,0), 1), calculate_distance_from_hq(inputs));
    let inputs = "R2, R1, L2, L2, L5, L2, L2, R1, L4, L5, L3, L5";
    assert_eq!(((2,1), 3), calculate_distance_from_hq(inputs));
}
