use itertools::Itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[cfg(test)]
use std::collections::HashSet;

pub fn run(filename: Option<&str>) {
    println!("Day 3: Squares With Three Sides");
    let file = File::open(filename.unwrap_or("data/day03.txt")).expect("file not found");
    let reader = BufReader::new(file);

    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    println!("part 1: {}", actual_triangle_count(&input));
    println!("part 2: {}", actual_vertical_triangle_count(&input));
}

/// The number of actual triangles when input is read horizontally.
pub fn actual_triangle_count(input: &[String]) -> usize {
    input
        .iter()
        .map(|input| parse_lengths(input))
        .filter(|triangle| is_triangle(triangle))
        .count()
}

/// The number of actual triangles when input is read vertically.
pub fn actual_vertical_triangle_count(input: &[String]) -> usize {
    read_vertical_triangles(input)
        .filter(|triangle| is_triangle(triangle))
        .count()
}

/// The number of vertical triangles
fn read_vertical_triangles<'a>(input: &'a [String]) -> impl Iterator<Item = Vec<usize>> + 'a {
    input
        .iter()
        .map(|input| parse_lengths(input))
        .tuples()
        .flat_map(|(a, b, c)| {
            vec![
                vec![a[0], b[0], c[0]],
                vec![a[1], b[1], c[1]],
                vec![a[2], b[2], c[2]],
            ]
        })
}

/// Turns a string of three numbers into a Vec of usizes.
fn parse_lengths(triangle: &str) -> Vec<usize> {
    triangle
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

/// Whether three numbers form a triangle.
fn is_triangle(numbers: &[usize]) -> bool {
    numbers[0] + numbers[1] > numbers[2]
        && numbers[1] + numbers[2] > numbers[0]
        && numbers[2] + numbers[0] > numbers[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalene() {
        let triangle = vec![3, 4, 5];
        assert!(is_triangle(&triangle));
    }

    #[test]
    fn test_equilateral() {
        let triangle = vec![3, 3, 3];
        assert!(is_triangle(&triangle));
    }

    #[test]
    fn test_isosceles() {
        let triangle = vec![3, 4, 4];
        assert!(is_triangle(&triangle));
    }

    #[test]
    fn test_non_triangle() {
        let triangle = vec![5, 10, 25];
        assert!(!is_triangle(&triangle));
    }

    #[test]
    fn vertical_parse_test() {
        let triangles = vec![
            "101 301 501".to_string(),
            "102 302 502".to_string(),
            "103 303 503".to_string(),
            "201 401 601".to_string(),
            "202 402 602".to_string(),
            "203 403 603".to_string(),
        ];
        let vertical_parse_expectation = hashset!{
            vec![101, 102, 103],
            vec![201, 202, 203],
            vec![301, 302, 303],
            vec![401, 402, 403],
            vec![501, 502, 503],
            vec![601, 602, 603],
        };
        assert_eq!(
            vertical_parse_expectation,
            read_vertical_triangles(&triangles).collect::<HashSet<Vec<usize>>>()
        );
    }

}
