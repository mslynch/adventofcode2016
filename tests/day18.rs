extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day18::run_with_rows;

#[test]
fn safe_tile_test() {
    let mut file = File::open("data/day18/test.dat").expect("File not found!");
    let result = run_with_rows(&mut file, 40, 40);
    assert_eq!("38", result.part1, "the number of safe tiles is computed");
}
