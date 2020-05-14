use crate::solution::Solution;
use im::hashmap::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Runs the solutions for day 10.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let (instructions, initial_state) = parse_input(&input);
    let (state, comparebot) = run_bots(instructions, initial_state, &[17, 61], None);
    let part1 = comparebot.unwrap().to_string();

    let bins = state.bins;
    let product = (0..3).fold(1, |product, bin_id| product * bins.get(&bin_id).unwrap());
    let part2 = product.to_string();

    Solution {
        title: "Balance Bots".to_string(),
        part1,
        part2,
    }
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

#[derive(Debug, PartialEq)]
struct State {
    bots: HashMap<usize, Vec<usize>>,
    bins: HashMap<usize, usize>,
}

/// Returns a tuple of the bot instructions and the initial state.
fn parse_input(input: &[String]) -> (HashMap<usize, Vec<Bot>>, State) {
    let (instructions, state_bots) =
        input
            .iter()
            .fold((HashMap::new(), HashMap::new()), |(bots, values), line| {
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
                            bots.update(
                                id,
                                vec![
                                    Bot {
                                        id: low_id,
                                        output_type: low_type,
                                    },
                                    Bot {
                                        id: high_id,
                                        output_type: high_type,
                                    },
                                ],
                            ),
                            values,
                        )
                    }
                    _value => {
                        let value = split.next().unwrap().parse::<usize>().unwrap();
                        // consume "goes to bot"
                        for _ in 0..3 {
                            split.next();
                        }
                        let bot = split.next().unwrap().parse::<usize>().unwrap();
                        (
                            bots,
                            values.update_with(bot, vec![value], |existing, _new| {
                                ordered_vec(value, existing[0])
                            }),
                        )
                    }
                }
            });
    (
        instructions,
        State {
            bots: state_bots,
            bins: HashMap::new(),
        },
    )
}

fn ordered_vec(a: usize, b: usize) -> Vec<usize> {
    if a < b {
        vec![a, b]
    } else {
        vec![b, a]
    }
}

/// Determines the end state and the bot responsible for comparing two different values
fn run_bots<S: ::std::hash::BuildHasher>(
    bot_instructions: HashMap<usize, Vec<Bot>, S>,
    state: State,
    comparebot_values: &[usize],
    bot_comparing_61_17: Option<usize>,
) -> (State, Option<usize>) {
    let bots = state.bots.clone();
    let bins = state.bins.clone();

    // find bots that are full and not full
    let (not_full, full): (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) =
        bots.into_iter().fold(
            (HashMap::new(), HashMap::new()),
            |(not_full, full), (bot, values)| match values.len() {
                2 => (not_full, full.update(bot, values)),
                _ => (not_full.update(bot, values), full),
            },
        );

    if full.is_empty() {
        (state, bot_comparing_61_17)
    } else {
        // distribute the full bots' values and find the comparebot if it exists
        let (non_empty, new_bins, new_comparebot) = full.iter().fold(
            (not_full, bins, bot_comparing_61_17),
            |(bot_merges, bin_merges, comparebot), (bot, values)| {
                // merge a bot's low and high into other bots
                let (new_bot_merges, new_bin_merges) = values
                    .iter()
                    .zip(bot_instructions.get(bot).unwrap().iter())
                    .fold(
                        (bot_merges, bin_merges),
                        |(bot_merge_acc, bin_merge_acc), (value, goes_to)| match goes_to.output_type
                        {
                            OutputType::Bot => (
                                bot_merge_acc.update_with(
                                    goes_to.id,
                                    vec![*value],
                                    |existing, _new| ordered_vec(*value, existing[0]),
                                ),
                                bin_merge_acc,
                            ),
                            OutputType::Bin => {
                                (bot_merge_acc, bin_merge_acc.update(goes_to.id, *value))
                            }
                        },
                    );

                let maybe_new_comparebot = comparebot.or_else(|| {
                    if values.as_slice() == comparebot_values {
                        Some(*bot)
                    } else {
                        None
                    }
                });

                (new_bot_merges, new_bin_merges, maybe_new_comparebot)
            },
        );

        // the previously full bots will have zero for the next iteration
        let empty = full.iter().map(|(bot, _value)| (*bot, vec![])).collect();

        let new_bots = non_empty.union(empty);
        let new_state = State {
            bots: new_bots,
            bins: new_bins,
        };

        run_bots(
            bot_instructions,
            new_state,
            comparebot_values,
            new_comparebot,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_bots_test() {
        let instructions = HashMap::new()
            .update(
                2,
                vec![
                    Bot {
                        id: 1,
                        output_type: OutputType::Bot,
                    },
                    Bot {
                        id: 0,
                        output_type: OutputType::Bot,
                    },
                ],
            )
            .update(
                1,
                vec![
                    Bot {
                        id: 1,
                        output_type: OutputType::Bin,
                    },
                    Bot {
                        id: 0,
                        output_type: OutputType::Bot,
                    },
                ],
            )
            .update(
                0,
                vec![
                    Bot {
                        id: 2,
                        output_type: OutputType::Bin,
                    },
                    Bot {
                        id: 0,
                        output_type: OutputType::Bin,
                    },
                ],
            );

        let state_bots = HashMap::new().update(2, vec![2, 5]).update(1, vec![3]);

        let initial_state = State {
            bots: state_bots,
            bins: HashMap::new(),
        };

        let expected_state = State {
            bots: HashMap::new()
                .update(2, vec![])
                .update(1, vec![])
                .update(0, vec![]),
            bins: HashMap::new().update(0, 5).update(1, 2).update(2, 3),
        };

        let (actual_state, comparebot) = run_bots(instructions, initial_state, &[2, 5], None);

        assert_eq!(expected_state, actual_state);
        assert_eq!(2, comparebot.unwrap());
    }
}
