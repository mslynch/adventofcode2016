extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day15::run;

#[test]
fn pad_test() {
    let mut file = File::open("data/day15/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("5", result.part1, "can retrieve capsule at time=5");
}
