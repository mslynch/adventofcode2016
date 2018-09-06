use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
    println!("Day 1: No Time for a Taxicab");
    day1();
}

fn day1() {
    let mut file = File::open("data/day1.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    let (ending_distance, revisited_distance) = blocks_away(&contents);
    println!("day 1: {}", ending_distance);
    println!("day 2: {}", revisited_distance.unwrap());
}

#[derive(Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone)]
struct Walk {
    position: Position,
    direction: Direction,
}

struct Instruction {
    relative_direction: char,
    distance: isize,
}

fn turn(direction: &Direction, relative_direction: char) -> Direction {
    match relative_direction {
        'L' => match direction {
            Direction::NORTH => Direction::WEST,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
            Direction::WEST => Direction::SOUTH,
        },
        _ => match direction {
            Direction::NORTH => Direction::EAST,
            Direction::SOUTH => Direction::WEST,
            Direction::EAST => Direction::SOUTH,
            Direction::WEST => Direction::NORTH,
        },
    }
}

fn do_walk(walk: &Walk, instruction: Instruction) -> Walk {
    let new_direction = turn(&walk.direction, instruction.relative_direction);
    let new_position = match new_direction {
        Direction::NORTH => Position {
            x: walk.position.x,
            y: walk.position.y + instruction.distance,
        },
        Direction::SOUTH => Position {
            x: walk.position.x,
            y: walk.position.y - instruction.distance,
        },
        Direction::EAST => Position {
            x: walk.position.x + instruction.distance,
            y: walk.position.y,
        },
        Direction::WEST => Position {
            x: walk.position.x - instruction.distance,
            y: walk.position.y,
        },
    };
    Walk {
        position: new_position,
        direction: new_direction,
    }
}

fn absolute_blocks(position: Position) -> isize {
    position.x.abs() + position.y.abs()
}

// fn x_updater()

/// Returns a tuple of the ending distance and the first revisited distance.
fn blocks_away(input: &str) -> (isize, Option<isize>) {
    let initial_walk = Walk {
        position: Position {
            x: 0,
            y: 0,
        },
        direction: Direction::NORTH,
    };

    let input_splitter = || { input.split(", ") };
    let size = input_splitter().count();

    let walk_result = input_splitter()
        .map(|i| {
            Instruction {
                relative_direction: i.chars().next().unwrap(),
                distance: i[1..].parse().unwrap()
            }
        })
        .scan(initial_walk, |walk, instruction| {
            let before_position = walk.position;
            let walked = do_walk(walk, instruction);
            let after_position = walked.position;


            // "no two closures, even if identical, have the same type", so we box them
            let x_updater = Box::new(|x| {
                Position {
                    x: x,
                    y: after_position.y,
                }
            });
            let y_updater = Box::new(|y| {
                Position {
                    x: after_position.x,
                    y: y,
                }
            });


            let mut asdf = (after_position.x - 1..before_position.x - 1).rev().map(*x_updater);
            asdf = (after_position.x - 1..before_position.x - 1).map(*x_updater);

            // let a: Iterator<isize> = match before_position.x.cmp(&after_position.x) {
            //     Ordering::Less => (before_position.x + 1..after_position.x + 1).map(*x_updater),
            //     Ordering::Greater => (after_position.x - 1..before_position.x - 1).rev().map(*x_updater),
            //     Ordering::Equal => match before_position.y.cmp(&after_position.y) {
            //         Ordering::Less => (before_position.y + 1..after_position.y + 1).map(*y_updater),
            //         Ordering::Greater => (after_position.y - 1..before_position.y - 1).rev().map(*y_updater),
            //         Ordering::Equal => (before_position.x + 1..after_position.x + 1).map(*y_updater),
            //         // Ordering::Equal => vec![after_position],
            //     },
            // };

            // if before_position.x != after_position.x {
            //
            // }
            *walk = walked;
            // copying the position
            Some(walked.position)
        });

    let mut visited = HashSet::new();
    // let (_, upper_bound_option) = walk_result.size_hint();
    // let upper_bound = upper_bound_option.unwrap();

    let mut first_visited_position = None;
    let mut final_position = None;

    for (i, position) in walk_result.enumerate() {
        if first_visited_position == None && !visited.insert(position) {
            first_visited_position = Some(position);
        }
        if i == size - 1 {
            final_position = Some(position);
        }
    }

    let ending_distance = match final_position {
        Some(position) => Some(absolute_blocks(position)),
        None => None,
    }.unwrap();

    let revisited_distance = match first_visited_position {
        Some(position) => Some(absolute_blocks(position)),
        None => None,
    };

    (ending_distance, revisited_distance)
}

#[test]
fn blocks_away_test_1() {
    let (result, _) = blocks_away("R2, L3");
    assert_eq!(result, 5);
}

#[test]
fn blocks_away_test_2() {
    let (result, _) = blocks_away("R2, R2, R2");
    assert_eq!(result, 2);
}

#[test]
fn blocks_away_test_3() {
    let (result, _) = blocks_away("R5, L5, R5, R3");
    assert_eq!(result, 12);
}

#[test]
fn test_revisit_blocks_away() {
    let (_, result) = blocks_away("R8, R4, R4, R8");
    assert_eq!(result.unwrap(), 4);
}


/////////
/////////
////f////
/////////
/////////