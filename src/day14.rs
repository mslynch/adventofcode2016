use crate::solution::Solution;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

/// Runs the solutions for day 14.
pub fn run(file: &mut File) -> Solution {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    Solution {
        title: "One-Time Pad".to_string(),
        part1: solve(&input, 1).to_string(),
        part2: solve(&input, 2017).to_string(),
    }
}

struct HashChars {
    triplet: Option<char>,
    quintuplet: Vec<char>,
}

pub fn solve(input: &str, hash_count: u32) -> usize {
    let mut hashchars_deque: VecDeque<HashChars> = (0..=1001)
        .map(|i| get_hashchars(input, i, hash_count))
        .collect();

    let mut found_pads_count = 0;
    let mut index = 0;
    while found_pads_count < 64 {
        if let Some(triplet) = hashchars_deque.front().unwrap().triplet {
            let quintuplet = hashchars_deque
                .iter()
                .skip(1)
                .find(|h| h.quintuplet.iter().any(|q| *q == triplet));
            if quintuplet.is_some() {
                found_pads_count += 1;
            }
        }

        index += 1;
        hashchars_deque.pop_front();
        hashchars_deque.push_back(get_hashchars(input, index + 1001, hash_count));
    }

    index - 1
}

fn get_hashchars(input: &str, index: usize, hash_count: u32) -> HashChars {
    let hash_input = format!("{}{}", input, index);
    let hash = hash(&hash_input, hash_count);
    HashChars {
        triplet: get_first_triplet(&hash),
        quintuplet: get_quintuplet_chars(&hash),
    }
}

fn hash(input: &str, mut hash_count: u32) -> String {
    let mut hash = input.to_string();
    while hash_count > 0 {
        hash = format!("{:x}", md5::compute(hash));
        hash_count -= 1;
    }
    hash
}

fn get_first_triplet(s: &str) -> Option<char> {
    let mut current_streak = 1;
    let mut chars = s.chars();
    let mut previous_char = chars.next().unwrap();
    for c in chars {
        current_streak = if c == previous_char {
            current_streak + 1
        } else {
            1
        };
        if current_streak == 3 {
            return Some(c);
        }
        previous_char = c;
    }
    None
}

fn get_quintuplet_chars(s: &str) -> Vec<char> {
    let mut current_streak = 1;
    let mut chars = s.chars();
    let mut previous_char = chars.next().unwrap();
    let mut quintuplet_chars = vec![];
    for c in chars {
        current_streak = if c == previous_char {
            current_streak + 1
        } else {
            1
        };
        if current_streak == 5 {
            quintuplet_chars.push(c);
        }
        previous_char = c;
    }
    quintuplet_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_n_in_a_row_test() {
        assert_eq!(Some('c'), get_first_triplet("abcccd"));
        assert_eq!(None, get_first_triplet("abccd"));
    }

    #[test]
    fn get_quintuplet_chars_test() {
        assert_eq!(vec!['b', 'c'], get_quintuplet_chars("aabbbbbaaacccccce"));
    }

    #[test]
    fn hash_test() {
        assert_eq!(
            "577571be4de9dcce85a041ba0410f29f".to_string(),
            hash("abc0", 1)
        );
        assert_eq!(
            "16062ce768787384c81fe17a7a60c7e3".to_string(),
            hash("abc0", 3)
        );
    }
}
