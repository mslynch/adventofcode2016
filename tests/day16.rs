extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day16::run_test;

#[test]
fn checksum_test() {
    let mut file = File::open("data/day16/test.dat").expect("File not found!");
    let result = run_test(&mut file);
    assert_eq!("01100", result.part1, "the correct checksum is calculated");
}
