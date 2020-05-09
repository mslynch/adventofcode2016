extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day24::run;

#[test]
fn traversal_test() {
    let mut file = File::open("data/day24/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!(
        "14", result.part1,
        "the fewest number of steps required to traverse is found"
    );
}
