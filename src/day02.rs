use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[rustfmt::skip]
pub const KEYPAD_NORMAL: &[char] = &[
    '1', '2', '3',
    '4', '5', '6',
    '7', '8', '9'
];

#[rustfmt::skip]
pub const KEYPAD_DIAMOND: &[char] = &[
    ' ', ' ', '1', ' ', ' ',
    ' ', '2', '3', '4', ' ',
    '5', '6', '7', '8', '9',
    ' ', 'A', 'B', 'C', ' ',
    ' ', ' ', 'D', ' ', ' ',
];

/// Runs the problems for day 2.
pub fn run(filename: Option<&str>) {
    println!("Day 2: Bathroom Security");
    let file = File::open(filename.unwrap_or("data/day02.txt")).expect("file not found");
    let reader = BufReader::new(file);

    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let code = decode_instructions(&input, KEYPAD_NORMAL, Coord { row: 1, col: 1 }, Coord::next);
    println!("part 1: {}", code);
    let diamond_code = decode_instructions(
        &input,
        KEYPAD_DIAMOND,
        Coord { row: 2, col: 0 },
        Coord::next_diamond,
    );
    println!("part 2: {}", diamond_code);
}

#[derive(Clone)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

/// Increments x if x < upper_bound, otherwise returns x.
fn increment(x: usize, upper_bound: usize) -> usize {
    if x < upper_bound {
        x + 1
    } else {
        x
    }
}

/// Increments x if x < upper_bound, otherwise returns x.
fn decrement(x: usize) -> usize {
    if x > 0 {
        x - 1
    } else {
        x
    }
}

impl Coord {
    /// The next coordinate after moving in the given direction using a normal keypad.
    pub fn next(&self, direction: char, size: usize) -> Coord {
        match direction {
            'R' => Coord {
                row: self.row,
                col: increment(self.col, size - 1),
            },
            'L' => Coord {
                row: self.row,
                col: decrement(self.col),
            },
            'U' => Coord {
                row: decrement(self.row),
                col: self.col,
            },
            _ => Coord {
                row: increment(self.row, size - 1),
                col: self.col,
            },
        }
    }

    /// The next coordinate after moving in the given direction using a diamond keypad.
    pub fn next_diamond(&self, direction: char, size: usize) -> Coord {
        let tentative_coord = self.next(direction, size);
        let half_size = size as isize / 2;
        let manhattan_from_center = (tentative_coord.row as isize - half_size).abs()
            + (tentative_coord.col as isize - half_size).abs();
        if manhattan_from_center > half_size {
            self.clone()
        } else {
            tentative_coord
        }
    }
}

/// Decodes the given instructions into the actual keycode.
pub fn decode_instructions<F>(
    instructions: &[String],
    keypad: &[char],
    start_coord: Coord,
    next_coord: F,
) -> String
where
    F: Fn(&Coord, char, usize) -> Coord,
{
    let size = (keypad.len() as f64).sqrt() as usize;
    instructions
        .iter()
        .scan(start_coord, |coord, instruction_line| {
            let new_coord = instruction_line.chars().fold(coord.clone(), {
                |fold_coord, direction| next_coord(&fold_coord, direction, size)
            });
            *coord = new_coord.clone();
            Some(new_coord)
        })
        .map(|coord| keypad[size * coord.row + coord.col].to_string())
        .collect()
}
