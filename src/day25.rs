use solution::Solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use assembunny::{parse_input, Instruction, Memory};

pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let instructions = parse_input(&input);

    Solution {
        title: "Clock Signal".to_string(),
        part1: find_lowest_clock_a(&instructions).to_string(),
        part2: "that's all there is :)".to_string(),
    }
}

fn find_lowest_clock_a(instructions: &[Instruction]) -> i32 {
    (1..)
        .find(|a| {
            let mut registers = HashMap::new();
            registers.insert('a', *a);
            registers.insert('b', 0);
            registers.insert('c', 0);
            registers.insert('d', 0);
            let mut memory = Memory::new(registers, instructions.to_vec());
            let iterations = 100;
            memory.provides_clock_signal(iterations)
        })
        .unwrap()
}
