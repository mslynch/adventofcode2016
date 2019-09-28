use std::fs::File;
use std::io::prelude::*;

use crypto::digest::Digest;
use crypto::md5::Md5;

/// Runs the problems for day 5.
pub fn run(filename: Option<&str>) {
    println!("Day 5: How About a Nice Game of Chess?");
    let mut file = File::open(filename.unwrap_or("data/day05.txt")).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("part 1: {}", generate_password(&contents));
    println!("part 2: {}", generate_password_2(&contents));
}

/// If the index and input make an interesting hash, return the character and position.
fn interesting_hash_digit(index: usize, input: &str) -> Option<(char, char)> {
    let mut md5 = Md5::new();
    let prehash = &format!("{}{}", input, index);
    md5.input_str(prehash);
    let result = md5.result_str();
    let mut hash_chars = result.chars();
    for _ in 0..5 {
        if hash_chars.next().unwrap() != '0' {
            return None;
        }
    }
    Some((hash_chars.next().unwrap(), hash_chars.next().unwrap()))
}

/// Generates passwords, with the 5th digit being the next character.
pub fn generate_password(input: &str) -> String {
    (0..)
        .filter_map(|i| interesting_hash_digit(i, input))
        .map(|(password_char, _)| password_char)
        .take(8)
        .collect()
}

/// Converts an array password to a String.
fn password_to_string(password: [Option<char>; 8]) -> String {
    password.iter().map(|c| c.unwrap_or('_')).collect()
}

/// Generates passwords, with the 5th digit being the position and the 6th being the character.
pub fn generate_password_2(input: &str) -> String {
    let mut password: [Option<char>; 8] = [None; 8];
    let mut digits_calculated = 0;
    'outer: for i in 0.. {
        if let Some((position_char, character)) = interesting_hash_digit(i, input) {
            let position = position_char.to_digit(16).unwrap() as usize;
            if position < 8 && password[position] == None {
                password[position] = Some(character);
                digits_calculated += 1;
            }
            if digits_calculated == 8 {
                break 'outer;
            }
        }
    }
    password_to_string(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interesting_hash_digit() {
        let (first, second) = interesting_hash_digit(3231929, "abc").unwrap();
        assert_eq!('1', first);
        assert_eq!('5', second);
    }
}
