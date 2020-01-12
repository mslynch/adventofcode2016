use solution::Solution;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Runs the solutions for day 20.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let mut ranges = parse_input(&input);

    ranges.sort_by(|(a_lower, _a_higher), (b_lower, _b_higher)| a_lower.cmp(b_lower));

    let (first, rest) = ranges.split_first().unwrap();
    let reduced_ranges = rest.iter().fold(vec![*first], |mut acc, (lower, upper)| {
        let (last_lower, last_upper) = *acc.last().unwrap();
        if *lower <= last_upper.checked_add(1).or(Some(last_upper)).unwrap() && *upper > last_upper
        {
            acc.remove(acc.len() - 1);
            acc.push((last_lower, *upper));
        } else if *lower > last_upper {
            acc.push((*lower, *upper));
        }
        acc
    });

    let (_first_range_lower, first_range_upper) = reduced_ranges.first().unwrap();
    let lowest_unblocked = first_range_upper + 1;

    let mut unblocked_count = 0;
    for i in 0..(reduced_ranges.len() - 1) {
        let (_a_lower, a_upper) = reduced_ranges[i];
        let (b_lower, _b_upper) = reduced_ranges[i + 1];
        unblocked_count += b_lower - a_upper - 1;
    }

    for (l, u) in reduced_ranges.iter() {
        println!("{}-{}", l, u);
    }

    Solution {
        title: "Firewall Rules".to_string(),
        part1: lowest_unblocked.to_string(),
        part2: unblocked_count.to_string(),
    }
}

fn parse_input(input: &[String]) -> Vec<(u32, u32)> {
    input
        .iter()
        .map(|ip_range| {
            let mut split = ip_range.split('-');
            let lower = split.next().unwrap().parse::<u32>().unwrap();
            let upper = split.next().unwrap().parse::<u32>().unwrap();
            (lower, upper)
        })
        .collect()
}
