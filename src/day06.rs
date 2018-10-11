use std::cmp::Ordering;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Runs the solution for day 6.
pub fn run(filename: Option<&str>) {
    println!("Day 6: Signals and Noise");
    let file = File::open(filename.unwrap_or("data/day06.txt")).expect("file not found");
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();

    println!("part 1: {}", error_correct(&input, &max_count_comparator));
    println!("part 2: {}", error_correct(&input, &min_count_comparator));
}

/// Returns the error-corrected version for a list of received messages.
pub fn error_correct<F>(input: &[String], comparator: F) -> String
where
    F: Fn(&(&char, &usize), &(&char, &usize)) -> Ordering,
{
    let mut char_counts: Vec<HashMap<char, usize>> = vec![HashMap::new(); input[0].len()];
    for string in input.iter() {
        for (i, character) in string.chars().enumerate() {
            char_counts[i]
                .entry(character)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    char_counts
        .iter()
        .map(|column_counts| {
            column_counts.iter().max_by(&comparator).map(|(character, _count)| character)
            .unwrap()
        }).collect()
}

pub fn max_count_comparator((_, count_a): &(&char, &usize), (_, count_b): &(&char, &usize)) -> Ordering {
    count_a.cmp(count_b)
}

pub fn min_count_comparator((_, count_a): &(&char, &usize), (_, count_b): &(&char, &usize)) -> Ordering {
    count_b.cmp(count_a)
}
