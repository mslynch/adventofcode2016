extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day20::run;

#[test]
fn lowest_unblocked_test() {
    let mut file = File::open("data/day20/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!(
        "3", result.part1,
        "the lowest-valued non-blocked ip is found"
    );
}

#[test]
fn unblocked_ip_count_test() {
    let mut file = File::open("data/day20/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("2", result.part2, "the number of unblocked ips is found");
}
