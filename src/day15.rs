use solution::Solution;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use fancy_regex::Regex;

/// Runs the solutions for day 15.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let mut discs = parse_input(&input);

    discs.sort_by(|a, b| b.position.cmp(&a.position));
    let part1 = first_time_to_press(&discs).to_string();
    discs.push(Disc {
        position: discs.len() as u32 + 1,
        size: 11,
        start: 0,
    });
    discs.sort_by(|a, b| b.position.cmp(&a.position));
    let part2 = first_time_to_press(&discs).to_string();

    Solution {
        title: "Timing is Everything".to_string(),
        part1,
        part2,
    }
}

struct Disc {
    position: u32,
    size: u32,
    start: u32,
}

impl Disc {
    fn is_open_starting_at(&self, time: u32) -> bool {
        (time + self.start + self.position) % self.size == 0
    }
}

fn first_time_to_press(discs: &[Disc]) -> u32 {
    (0..)
        .find(|i| discs.iter().all(|disc| disc.is_open_starting_at(*i)))
        .unwrap()
}

fn parse_input(input: &[String]) -> Vec<Disc> {
    let size_re = Regex::new(r"(?<=\bhas\s)(\w+)").unwrap();
    let start_re = Regex::new(r"(?<=\bposition\s)(\w+)").unwrap();
    let get_number = |regex: &Regex, input: &str| {
        regex
            .captures(input)
            .unwrap()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap()
    };

    input
        .iter()
        .enumerate()
        .map(|(i, line)| Disc {
            size: get_number(&size_re, line),
            start: get_number(&start_re, line),
            position: i as u32 + 1,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_open_at_test() {
        let disc_1 = Disc {
            position: 1,
            size: 5,
            start: 4,
        };
        assert!(disc_1.is_open_starting_at(0));
        assert!(disc_1.is_open_starting_at(5));

        let disc_2 = Disc {
            position: 2,
            size: 2,
            start: 1,
        };
        assert!(!disc_2.is_open_starting_at(0));
        assert!(disc_2.is_open_starting_at(5));
    }
}
