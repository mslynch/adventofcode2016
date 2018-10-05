use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

/// Runs the solutions for day 8.
pub fn run(filename: Option<&str>) {
    println!("Day 10: Balance Bots");
    let file = File::open(filename.unwrap_or("data/day10.txt")).expect("file not found");
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();

    // let (lit, screen) = run_instructions(&input);
    // println!("part 1: {}", lit);
    // println!("part 2: ");
    // display(&screen);
}

enum OutputType {
    Bot,
    Bin,
}

struct Bot {
    id: usize,
    output_type: OutputType,
}

fn parse_input(input: &[String]) -> HashMap<usize, (Bot, Bot)> {
    let mut bots: HashMap::new();
    // let mut bot_values: HashMap::new();
    for split in input.iter().map(|line| line.split(' ')) {
        match split.next().unwrap() {
            "bot" => {
                let id = split.next().unwrap().parse::<usize>().unwrap();
                // consume "gives low to"
                for _ in 0..3 {
                    split.next();
                }
                let low_type = match split.next().unwrap() {
                    "bot" => OutputType::Bot,
                    _ => OutputType::Bin,
                };
                let low_id = split.next().unwrap().parse::<usize>().unwrap();
                // consume "and high to"
                for _ in 0..3 {
                    split.next();
                }
                let high_type = match split.next().unwrap() {
                    "bot" => OutputType::Bot,
                    _ => OutputType::Bin,
                };
                let high_id = split.next().unwrap().parse::<usize>().unwrap();
                bots.insert(
                    id,
                    (
                        Bot {
                            id: low_id,
                            output_type: low_type,
                        },
                        Bot {
                            id: high_id,
                            output_type: high_type,
                        },
                    ),
                );
            }
            _ => {
                
            }
        }
    }

    3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_row_test_2() {
        // let mut screen = [
        //     '.', '.', '.', '.', '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.',
        //     '.', '.', '.', '.',
        // ];
        // run_instruction(
        //     &mut screen,
        //     &Dimensions { x: 7, y: 3 },
        //     "rotate column x=1 by 1",
        // );
        // let expected = [
        //     '.', '#', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '.', '.', '#', '.',
        //     '.', '.', '.', '.',
        // ];
        // assert_eq!(expected, screen);
    }

}
