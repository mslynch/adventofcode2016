use std::fs::File;
use std::io::prelude::*;

use crypto::digest::Digest;
use crypto::md5::Md5;

/// Runs the problems for day 1.
pub fn run(filename: Option<&str>) {
    println!("Day 1: No Time for a Taxicab");
    let mut file = File::open(filename.unwrap_or("data/day05.txt")).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("part 1: {}", generate_password(&contents));
}

fn interesting_hash(index: usize, input: &str) -> Option<char> {
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
    hash_chars.next()
}

pub fn generate_password(input: &str) -> String {
    (0..)
        .filter_map(|i| interesting_hash(i, input))
        .take(8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interesting_hash() {
        assert_eq!('1', interesting_hash(3231929, "abc").unwrap());
    }

}
