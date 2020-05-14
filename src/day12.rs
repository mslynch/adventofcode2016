use crate::solution::Solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::assembunny::{parse_input, Instruction, Memory};

/// Runs the solutions for day 12.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let instructions = parse_input(&input);

    let part1 = run_and_get_register_a(instructions.clone(), 0).to_string();
    let part2 = run_and_get_register_a(instructions, 1).to_string();

    Solution {
        title: "Leonardo's Monorail".to_string(),
        part1,
        part2,
    }
}

fn run_and_get_register_a(instructions: Vec<Instruction>, initial_c: i32) -> i32 {
    let mut registers = HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);
    registers.insert('c', initial_c);
    registers.insert('d', 0);
    let mut memory = Memory::new(registers, instructions);
    memory.run_instructions();
    *memory.registers.get(&'a').unwrap()
}
