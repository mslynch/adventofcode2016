use std::fs::File;
use std::io::prelude::*;
use std::str;

/// Runs the solutions for day 9.
pub fn run(filename: Option<&str>) {
    println!("Day 9: Explosives in Cyberspace");
    let mut file = File::open(filename.unwrap_or("data/day09.txt")).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("part 1: {}", decompress(&contents).len());
    println!("part 2: {}", decompress_recursive(&contents, 1));
}

/// Decompresses a compressed string without expanding inner compressions.
pub fn decompress(input: &str) -> String {
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
pub fn decompress_recursive(input: &str, multiplier: usize) -> usize {
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
