extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day09::run;

#[test]
fn decompression_test() {
    let mut file = File::open("data/day09/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("18", result, "decompressed length is calculated");
}

#[test]
fn recursive_decompression_test() {
    let mut file = File::open("data/day09/test-recursive.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!(
        "445", result,
        "decompressed length is recursively calculated"
    );
}
