extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day12::run;

#[test]
fn min_steps_test() {
    let mut file = File::open("data/day12/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("42", result, "instructions are executed");
}
