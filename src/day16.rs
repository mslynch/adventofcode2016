use solution::Solution;
use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

/// Runs the solutions for day 16.
pub fn run(file: &mut File) -> Solution {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    Solution {
        title: "Dragon Checksum".to_string(),
        part1: curve_and_checksum(&input, 272),
        part2: curve_and_checksum(&input, 35_651_584),
    }
}

pub fn run_test(file: &mut File) -> Solution {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    Solution {
        title: "Dragon Checksum".to_string(),
        part1: curve_and_checksum(&input, 20),
        part2: curve_and_checksum(&input, 20),
    }
}

fn curve_and_checksum(input: &str, disk_space: usize) -> String {
    let mut curve = input.to_string();
    while curve.len() < disk_space {
        curve = dragon_curve(&curve);
    }
    let mut checksum_input: String = curve.chars().take(disk_space).collect();
    while checksum_input.len() % 2 == 0 {
        checksum_input = checksum(&checksum_input);
    }
    checksum_input
}

fn checksum(input: &str) -> String {
    input
        .chars()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let a = chunk.next();
            let b = chunk.next();
            if a == b {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn dragon_curve(a: &str) -> String {
    let b: String = a
        .chars()
        .rev()
        .map(|c| match c {
            '0' => '1',
            _ => '0',
        })
        .collect();
    format!("{}0{}", a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dragon_curve_test() {
        assert_eq!(dragon_curve("1"), "100".to_string());
        assert_eq!(dragon_curve("0"), "001".to_string());
        assert_eq!(dragon_curve("11111"), "11111000000".to_string());
        assert_eq!(
            dragon_curve("111100001010"),
            "1111000010100101011110000".to_string()
        );
    }

    #[test]
    fn checksum_test() {
        assert_eq!(checksum("110010110100"), "110101".to_string());
        assert_eq!(checksum("110101"), "100".to_string());
    }
}
