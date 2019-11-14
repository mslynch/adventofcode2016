extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day13::run_test;

#[test]
fn min_steps_test() {
    let mut file = File::open("data/day13/test.dat").expect("File not found!");
    let result = run_test(&mut file).part1;
    assert_eq!("11", result, "instructions are executed");
}
