use solution::Solution;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

/// Runs the solutions for day 19.
pub fn run(file: &mut File) -> Solution {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let input_num = input.parse().unwrap();

    Solution {
        title: "Like a Rogue".to_string(),
        part1: get_last_elf(input_num).to_string(),
        part2: get_last_elf_opposite(input_num).to_string(),
    }
}

fn get_last_elf(num_elves: usize) -> usize {
    let mut elves = (1..=num_elves).collect::<VecDeque<usize>>();
    while elves.len() > 1 {
        let thief = elves.pop_front().unwrap();
        elves.pop_front().unwrap();
        elves.push_back(thief);
    }
    elves.pop_front().unwrap()
}

fn get_last_elf_opposite(num_elves: usize) -> usize {
    let mut elves_back = (1..=num_elves).collect::<VecDeque<usize>>();
    let mut elves_front = elves_back.split_off(elves_back.len() / 2);
    std::mem::swap(&mut elves_front, &mut elves_back);

    while elves_front.len() + elves_back.len() > 1 {
        let thief = elves_front.pop_front().unwrap();
        elves_back.pop_front().unwrap();
        if (elves_front.len() + elves_back.len()) % 2 == 1 {
            elves_front.push_back(elves_back.pop_front().unwrap());
        }
        elves_back.push_back(thief);
    }

    elves_front
        .pop_front()
        .or_else(|| elves_back.pop_front())
        .unwrap()
}
