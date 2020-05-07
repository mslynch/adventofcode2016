extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day23::run;

#[test]
fn assembly_test() {
    let mut file = File::open("data/day23/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("3", result.part1, "the final register of a is found");
}

// #[test]
// fn unscramble_test() {
//     let mut file = File::open("data/day21/test.dat").expect("File not found!");
//     let result = run_with_string(&mut file, "abcde", "decab");
//     assert_eq!("abcde", result.part2, "the string is unscrambled");
// }
