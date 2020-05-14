use crate::solution::Solution;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

/// Runs the solutions for day 8.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();

    let (lit, screen) = run_instructions(&input);
    display(&screen); // part 2

    Solution {
        title: "Two-Factor Authentication".to_string(),
        part1: lit.to_string(),
        part2: display(&screen),
    }
}

struct Dimensions {
    x: usize,
    y: usize,
}

/// Runs the instructions and returns the number of filled pixels, along with the screen.
fn run_instructions(instructions: &[String]) -> (usize, [char; 300]) {
    let mut screen = ['.'; 300];
    let dimensions = Dimensions { x: 50, y: 6 };
    for instruction in instructions {
        run_instruction(&mut screen, &dimensions, instruction);
    }
    (screen.iter().filter(|pixel| **pixel == '#').count(), screen)
}

/// Prints the screen.
fn display(screen: &[char]) -> String {
    let mut output = "".to_string();
    for row in 0..6 {
        for col in 0..50 {
            let c = match screen[row * 50 + col] {
                '#' => '█',
                _ => ' ',
            };
            output.push(c);
        }
        output.push('\n');
    }
    output
}

/// Executes a fill instruction.
fn fill(screen: &mut [char], dimensions: &Dimensions, width: usize, height: usize) {
    for row in 0..height {
        for col in 0..width {
            screen[row * dimensions.x + col] = '#';
        }
    }
}

/// Executes a rotate instruction.
fn rotate<I>(screen: &mut [char], index_range: I, dimension: usize, amount: usize)
where
    I: Iterator<Item = usize> + Clone,
{
    let index_range_clone = index_range.clone();
    let new_row: Vec<char> = index_range
        .map(|i| screen[i])
        .cycle()
        .skip(dimension - amount)
        .take(dimension)
        .collect();
    for (i, val) in index_range_clone.zip(new_row.iter()) {
        screen[i] = *val;
    }
}

/// Parses a rotate instruction into the row/column and amount to rotate.
fn parse_rotate_instruction<'a, I>(mut instruction: I) -> (usize, usize)
where
    I: Iterator<Item = &'a str>,
{
    let row_or_column_number = instruction
        .next()
        .unwrap()
        .chars()
        .skip(2)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    // skip "by"
    instruction.next();
    let amount = instruction.next().unwrap().parse::<usize>().unwrap();
    (row_or_column_number, amount)
}

/// Applies the given string instruction to the screen.
fn run_instruction(screen: &mut [char], dimensions: &Dimensions, instruction: &str) {
    let mut split = instruction.split(' ');
    match split.next().unwrap() {
        "rect" => {
            let mut split_width_height = split
                .next()
                .map(|s| s.split('x'))
                .map(|split| split.map(|dimension| dimension.parse::<usize>()))
                .unwrap();
            let mut next_int = || split_width_height.next().unwrap().unwrap();
            fill(screen, dimensions, next_int(), next_int());
        }
        _ => match split.next().unwrap() {
            "row" => {
                let (row_num, amount) = parse_rotate_instruction(split);
                let index_range = dimensions.x * row_num..dimensions.x * (row_num + 1);
                rotate(screen, index_range, dimensions.x, amount);
            }
            _ => {
                let (col_num, amount) = parse_rotate_instruction(split);
                let index_range = (col_num..screen.len()).step_by(dimensions.x);
                rotate(screen, index_range, dimensions.y, amount);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_test() {
        let mut screen = ['.'; 21];
        run_instruction(&mut screen, &Dimensions { x: 7, y: 3 }, "rect 3x2");
        let expected = [
            '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.',
        ];
        assert_eq!(expected, screen);
    }

    #[test]
    fn rotate_col_test() {
        let mut screen = [
            '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.',
        ];
        run_instruction(
            &mut screen,
            &Dimensions { x: 7, y: 3 },
            "rotate column x=1 by 1",
        );
        let expected = [
            '#', '.', '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.',
            '.', '.', '.', '.',
        ];
        assert_eq!(expected, screen);
    }

    #[test]
    fn rotate_row_test_1() {
        let mut screen = [
            '#', '.', '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.',
            '.', '.', '.', '.',
        ];
        run_instruction(
            &mut screen,
            &Dimensions { x: 7, y: 3 },
            "rotate row y=0 by 4",
        );
        let expected = [
            '.', '.', '.', '.', '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.',
            '.', '.', '.', '.',
        ];
        assert_eq!(expected, screen);
    }

    #[test]
    fn rotate_row_test_2() {
        let mut screen = [
            '.', '.', '.', '.', '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.',
            '.', '.', '.', '.',
        ];
        run_instruction(
            &mut screen,
            &Dimensions { x: 7, y: 3 },
            "rotate column x=1 by 1",
        );
        let expected = [
            '.', '#', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '.', '#', '.',
            '.', '.', '.', '.',
        ];
        assert_eq!(expected, screen);
    }
}
