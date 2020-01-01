extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day19::run;

#[test]
fn safe_tile_test_next_elf() {
    let mut file = File::open("data/day19/test.dat").expect("File not found!");
    let result = run(&mut file);
    assert_eq!(
        "3", result.part1,
        "the elf that gets all the presents is found when stealing from the next elf"
    );
    // assert_eq!("2", result.part2, "the elf that gets all the presents is found when stealing from the opposite elf");
}

#[test]
fn safe_tile_test_opposite_elf() {
    let mut file = File::open("data/day19/test.dat").expect("File not found!");
    let result = run(&mut file);
    // assert_eq!("3", result.part1, "the elf that gets all the presents is found when stealing from the next elf");
    assert_eq!(
        "2", result.part2,
        "the elf that gets all the presents is found when stealing from the opposite elf"
    );
}
