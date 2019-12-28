use solution::Solution;
use std::fs::File;
use std::io::prelude::*;

/// Runs the solutions for day 18.
pub fn run(file: &mut File) -> Solution {
    run_with_rows(file, 40, 400_000)
}

pub fn run_with_rows(file: &mut File, num_rows_part1: usize, num_rows_part2: usize) -> Solution {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    Solution {
        title: "Like a Rogue".to_string(),
        part1: get_safe_tile_count(&input, num_rows_part1).to_string(),
        part2: get_safe_tile_count(&input, num_rows_part2).to_string(),
    }
}


fn get_tile(previous_row: &str, position: usize) -> char {
    let padded = format!(".{}.", previous_row);
    match &padded[position..=position + 2] {
        "^^." => '^',
        ".^^" => '^',
        "^.." => '^',
        "..^" => '^',
        _ => '.',
    }
}

fn get_next_row(previous_row: &str) -> String {
    (0..previous_row.len())
        .map(|i| get_tile(previous_row, i))
        .collect()
}

fn get_safe_tile_count(first_row: &str, mut num_rows: usize) -> usize {
    let mut row = first_row.to_string();
    let mut count = 0;
    while num_rows > 0 {
        count += row.chars().filter(|c| *c == '.').count();
        row = get_next_row(&row);
        num_rows -= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tile_test() {
        assert_eq!(get_tile("..^^.", 0), '.');
        assert_eq!(get_tile("..^^.", 1), '^');
        assert_eq!(get_tile("..^^.", 2), '^');
        assert_eq!(get_tile("..^^.", 3), '^');
    }

    #[test]
    fn get_next_row_test() {
        assert_eq!(get_next_row("..^^."), ".^^^^".to_string());
        assert_eq!(get_next_row(".^^^^"), "^^..^".to_string());
    }
}
