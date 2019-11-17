extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day14::run;

#[test]
fn pad_test() {
    let mut file = File::open("data/day14/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("22728", result.part1, "64th one-time pad is calculated");
    assert_eq!(
        "22551", result.part2,
        "64th one-time pad is calculated hashing 2017 times"
    );
}
