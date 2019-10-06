extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day08::run;

#[test]
fn lit_after_instructions_test() {
    let mut file = File::open("data/day08/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("6", result, "lit pixels are counted correctly");
}
