use crate::solution::Solution;
use std::fs::File;
use std::io::prelude::*;
use std::str;

/// Runs the solutions for day 9.
pub fn run(file: &mut File) -> Solution {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let part1 = decompress(&contents).len().to_string();
    let part2 = decompress_recursive(&contents, 1).to_string();

    Solution {
        title: "Explosives in Cyberspace".to_string(),
        part1,
        part2,
    }
}

/// Decompresses a compressed string without expanding inner compressions.
fn decompress(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars();
    loop {
        match chars.next() {
            Some('(') => {
                output.push_str(&decompress_marker(&mut chars));
            }
            Some(ch) => {
                output.push(ch);
            }
            None => break,
        }
    }
    output
}

/// Extracts a marker (number of compressed chars, times to repeat) from the input chars.
fn extract_marker<I>(chars: &mut I) -> (usize, usize)
where
    I: Iterator<Item = char>,
{
    let mut marker = String::new();
    loop {
        match chars.next() {
            Some(')') => break,
            Some(ch) => marker.push(ch),
            None => panic!("Unexpected end of input!"),
        }
    }
    let mut split_marker = marker.split('x');
    let num_chars = split_marker.next().unwrap().parse::<usize>().unwrap();
    let repeat = split_marker.next().unwrap().parse::<usize>().unwrap();
    (num_chars, repeat)
}

/// Expands a marker from a String.
fn decompress_marker<I>(chars: &mut I) -> String
where
    I: Iterator<Item = char>,
{
    let (num_chars, repeat) = extract_marker(chars);
    let mut to_repeat = String::new();
    for _ in 0..num_chars {
        to_repeat.push(chars.next().unwrap());
    }
    to_repeat.repeat(repeat)
}

/// The length of the full decompression of a compressed string.
fn decompress_recursive(input: &str, multiplier: usize) -> usize {
    let mut length = 0;
    let mut chars = input.chars();
    loop {
        match chars.next() {
            Some('(') => {
                let (num_chars, repeat) = extract_marker(&mut chars);
                let mut contains_marker = false;
                let mut collected = String::new();
                for _ in 0..num_chars {
                    let ch = chars.next().unwrap();
                    if ch == '(' {
                        contains_marker = true;
                    }
                    collected.push(ch);
                }
                if contains_marker {
                    length += decompress_recursive(&collected, multiplier * repeat);
                } else {
                    length += multiplier * repeat * collected.len();
                }
            }
            Some(_) => length += multiplier,
            None => break,
        }
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompression_test_1() {
        assert_eq!("ADVENT", decompress("ADVENT"));
    }

    #[test]
    fn decompression_test_2() {
        assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
    }

    #[test]
    fn decompression_test_3() {
        assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
    }

    #[test]
    fn decompression_test_4() {
        assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
    }

    #[test]
    fn decompression_test_5() {
        assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
    }

    #[test]
    fn decompression_test_6() {
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn recursive_decompression_test_1() {
        assert_eq!(9, decompress_recursive("(3x3)XYZ", 1));
    }

    #[test]
    fn recursive_decompression_test_2() {
        assert_eq!(20, decompress_recursive("X(8x2)(3x3)ABCY", 1));
    }

    #[test]
    fn recursive_decompression_test_3() {
        assert_eq!(
            241920,
            decompress_recursive("(27x12)(20x12)(13x14)(7x10)(1x12)A", 1)
        );
    }

    #[test]
    fn recursive_decompression_test_4() {
        assert_eq!(
            445,
            decompress_recursive(
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                1
            )
        );
    }
}
