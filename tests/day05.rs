extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day05::run;

#[test]
fn password_generation_test() {
    let mut file = File::open("data/day05/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("18f47a30", result, "password is generated");
}

#[test]
fn password_generation_2_test() {
    let mut file = File::open("data/day05/test.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!("05ace8e3", result, "password is generated");
}
