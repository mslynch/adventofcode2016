use crate::solution::Solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::assembunny::{parse_input, Instruction, Memory};

pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let instructions = parse_input(&input);

    let part1 = run_and_get_register_a(instructions.clone(), 7).to_string();
    let part2 = run_and_get_register_a(instructions, 12).to_string();

    Solution {
        title: "Safe Cracking".to_string(),
        part1,
        part2,
    }
}

fn run_and_get_register_a(instructions: Vec<Instruction>, initial_a: i32) -> i32 {
    let mut registers = HashMap::new();
    registers.insert('a', initial_a);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
    let mut memory = Memory::new(registers, instructions);
    memory.run_instructions();
    *memory.registers.get(&'a').unwrap()
}
