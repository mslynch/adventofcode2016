extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day17::run;

#[test]
fn shortest_path_test_1() {
    let mut file = File::open("data/day17/test1.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("DDRRRD", result.part1, "the shortest path is found");
}

#[test]
fn shortest_path_test_2() {
    let mut file = File::open("data/day17/test2.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("DDUDRLRRUDRD", result.part1, "the shortest path is found");
}

#[test]
fn shortest_path_test_3() {
    let mut file = File::open("data/day17/test3.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!(
        "DRURDRUDDLLDLUURRDULRLDUUDDDRR", result.part1,
        "the shortest path is found"
    );
}

#[test]
fn longest_path_test_1() {
    let mut file = File::open("data/day17/test1.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("370", result.part2, "the longest path is found");
}

#[test]
fn longest_path_test_2() {
    let mut file = File::open("data/day17/test2.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("492", result.part2, "the longest path is found");
}

#[test]
fn longest_path_test_3() {
    let mut file = File::open("data/day17/test3.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!("830", result.part2, "the longest path is found");
}
