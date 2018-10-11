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
    let (instructions, mut initial_state) = parse_input(&input);
    // let (lit, screen) = run_instructions(&input);
    println!("part 1: {}", run_bots(&instructions, &mut initial_state));
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

/// Returns a tuple of the bot instructions and the initial state.
fn parse_input(input: &[String]) -> (HashMap<usize, (Bot, Bot)>, HashMap<usize, Vec<usize>>) {
    let mut bots = HashMap::new();
    let mut values = HashMap::new();
    for line in input.iter() {
        let mut split = line.split(' ');
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
            _value => {
                let value = split.next().unwrap().parse::<usize>().unwrap();
                // consume "goes to bot"
                for _ in 0..3 {
                    split.next();
                }
                let bot = split.next().unwrap().parse::<usize>().unwrap();
                values.insert(bot, vec![value]);
            }
        }
    }
    (bots, values)
}

struct BotState {

}

fn run_bots(
    bot_instructions: &HashMap<usize, (Bot, Bot)>,
    bot_state: &mut HashMap<usize, Vec<usize>>,
) -> usize {
    // let current
    let mut bot_comparing_61_17 = None;
    loop {

        // bot_state.retain(|&bot_id, ref mut values| {
        //     if values.len() > 1 {
        //         let low = values[0];
        //         let high = values[1];
        //         if low == 17 && high == 61 {
        //             bot_comparing_61_17 = Some(bot_id);
        //         }
        //         true
        //     } else {
        //         false
        //     }
        // });
        // if bot_state.iter()
    }
    bot_comparing_61_17.unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_row_test_2() {

        // assert_eq!(expected, screen);
    }

}
