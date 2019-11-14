use solution::Solution;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

/// Runs the solutions for day 13.
pub fn run(file: &mut File) -> Solution {
    solve(file, (31, 39))
}

pub fn run_test(file: &mut File) -> Solution {
    solve(file, (7, 4))
}

pub fn solve(file: &mut File, target_position: (u32, u32)) -> Solution {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let input = contents.parse::<u32>().unwrap();

    let (part1, part2) = fewest_steps_to_reach(input, target_position);

    Solution {
        title: "A Maze of Twisty Little Cubicles".to_string(),
        part1: part1.to_string(),
        part2: part2.to_string(),
    }
}

fn fewest_steps_to_reach(input: u32, target_position: (u32, u32)) -> (u32, usize) {
    let mut current = HashSet::new();
    current.insert((1, 1));
    let mut visited = current.clone();
    let mut steps = 0;
    let mut distinct_location_count = 0;
    while !visited.contains(&target_position) {
        current = current
            .iter()
            .flat_map(|coord| get_adjacent_coords(*coord))
            .filter(|coord| is_open(input, *coord) && !visited.contains(coord))
            .collect();
        visited.extend(&current);
        steps += 1;
        if steps <= 50 {
            distinct_location_count = visited.len();
        }
    }
    (steps, distinct_location_count)
}

fn get_adjacent_coords((current_x, current_y): (u32, u32)) -> Vec<(u32, u32)> {
    let mut adjacent = vec![(current_x + 1, current_y), (current_x, current_y + 1)];
    if current_x > 0 {
        adjacent.push((current_x - 1, current_y));
    }
    if current_y > 0 {
        adjacent.push((current_x, current_y - 1));
    }
    adjacent
}

fn is_open(input: u32, position: (u32, u32)) -> bool {
    let (x, y) = position;
    let computed = x * x + 3 * x + 2 * x * y + y + y * y + input;
    let binary_string = format!("{:b}", computed);
    binary_string.chars().filter(|c| *c == '1').count() % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_open() {
        assert!(is_open(10, (0, 0)));
        assert!(!is_open(10, (1, 0)));
        assert!(is_open(10, (2, 0)));
        assert!(!is_open(10, (3, 0)));
        assert!(!is_open(10, (4, 0)));
        assert!(!is_open(10, (5, 0)));
        assert!(!is_open(10, (6, 0)));
        assert!(is_open(10, (7, 0)));
        assert!(!is_open(10, (8, 0)));
        assert!(!is_open(10, (9, 0)));

        assert!(is_open(10, (0, 1)));
        assert!(is_open(10, (1, 1)));
        assert!(!is_open(10, (2, 1)));
        assert!(is_open(10, (3, 1)));
        assert!(is_open(10, (4, 1)));
        assert!(!is_open(10, (5, 1)));
        assert!(is_open(10, (6, 1)));
        assert!(is_open(10, (7, 1)));
        assert!(is_open(10, (8, 1)));
        assert!(!is_open(10, (9, 1)));
    }
}
