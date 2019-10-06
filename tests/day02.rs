extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day02::run;

#[test]
fn decode_test() {
    let mut file = File::open("data/day02/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("1985", result, "bathroom code is decoded");
}

#[test]
fn diamond_decode_test() {
    let mut file = File::open("data/day02/test.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!(
        "5DB3", result,
        "bathroom code is decoded with a diamond keypad"
    );
}
