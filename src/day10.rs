// use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use im::hashmap::HashMap;
// use itertools::Itertools;

/// Runs the solutions for day 10.
pub fn run(filename: Option<&str>) {
    println!("Day 10: Balance Bots");
    let file = File::open(filename.unwrap_or("data/day10.txt")).expect("file not found");
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let (instructions, mut initial_state) = parse_input(&input);
    // let (lit, screen) = run_instructions(&input);
    println!("part 1: {}", run_bots(&instructions, &mut initial_state, None));
    // println!("part 2: ");
    // display(&screen);
}

#[derive(Clone)]
enum OutputType {
    Bot,
    Bin,
}

#[derive(Clone)]
struct Bot {
    id: usize,
    output_type: OutputType,
}

/// Returns a tuple of the bot instructions and the initial state.
fn parse_input(input: &[String]) -> (HashMap<usize, Vec<Bot>>, HashMap<usize, Vec<usize>>) {
    input.iter().fold((HashMap::new(), HashMap::new()), |(bots, values), line| {
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
                (
                    bots.update(id, 
                        vec![
                            Bot {
                                id: low_id,
                                output_type: low_type,
                            },
                            Bot {
                                id: high_id,
                                output_type: high_type,
                            },
                        ]), 
                        values)

            }
            _value => {
                let value = split.next().unwrap().parse::<usize>().unwrap();
                // consume "goes to bot"
                for _ in 0..3 {
                    split.next();
                }
                let bot = split.next().unwrap().parse::<usize>().unwrap();
                (bots, values.update_with(bot, vec![value], |existing, _new| ordered_vec(value, existing[0])))
            }
        }
    })
}


fn ordered_vec(a: usize, b: usize) -> Vec<usize> {
    if a < b {
        vec![a, b]
    } else {
        vec![b, a]
    }
}

fn run_bots(
    bot_instructions: &HashMap<usize, Vec<Bot>>,
    bot_state: &HashMap<usize, Vec<usize>>,
    bot_comparing_61_17: Option<usize>
) -> usize {

    // find bots that are full and not full
    let (not_full, full) = bot_state
        .iter()
        .fold((HashMap::new(), HashMap::new()), |(not_full, full), (bot, values)| {
            match values.len() {
                2 => (not_full, full.update(bot, values)),
                _ => (not_full.update(bot, values), full),
            }
        });

    // compute bots that have values after this iteration
    let (non_empty, new_compare_bot) = full.iter().fold((not_full, bot_comparing_61_17), |(merges, compare_bot), (bot, values)| {
        // merge the low and high values into the single-value bots
        // let x: usize = bot_instructions.get(bot).unwrap();
        let new_merges = values.iter().zip(bot_instructions.get(bot).unwrap().iter()).fold(merges, |merge_acc, (value, goes_to)| {
            merge_acc.update_with(&goes_to.id, &vec![*value], |existing, _new| &ordered_vec(*value, existing[0]))
        });

        let maybe_new_compare_bot = compare_bot.or_else(|| {
            if values[0] == 17 && values[1] == 61 {
                Some(**bot)
            } else {
                None
            }
        });//});

        (new_merges, maybe_new_compare_bot)
    });

    // the previously full bots will have zero for the next iteration
    let empty = full.iter().map(|(bot, _value)| (*bot, &vec![])).collect();

    let new_bot_state = non_empty.union(empty);

    // let states_by_length = bot_state.iter().group_by(|(_id, low_and_high)| low_and_high.len());

    // bot_comparing_61_17.unwrap();
    8
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_row_test_2() {

        // assert_eq!(expected, screen);
    }

}
