extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day11::run;

#[test]
fn min_steps_test() {
    // F4 .  .  .  .  .
    // F3 .  .  .  LG .
    // F2 .  HG .  .  .
    // F1 E  .  HM .  LM

    let mut file = File::open("data/day11/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!(
        "11", result,
        "objects moved to fourth floor in minimum number of steps"
    );
}
