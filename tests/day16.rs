extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day15::run;

#[test]
fn pad_test() {
    let mut file = File::open("data/day15/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("01100", result.part1, "the correct checksum is calculated");
    // assert_eq!(
    //     "22551", result.part2,
    //     "64th one-time pad is calculated hashing 2017 times"
    // );
}
