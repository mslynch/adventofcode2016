use solution::Solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

/// Runs the solutions for day 12.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let instructions = parse_input(&input);

    let part1 = run_and_get_register_a(&instructions).to_string();
    let part2 = run_and_get_register_a_part2(&instructions).to_string();

    Solution {
        title: "Leonardo's Monorail".to_string(),
        part1,
        part2,
    }
}

fn run_and_get_register_a(instructions: &[Instruction]) -> i32 {
    let mut memory = Memory::new();
    memory.run_instructions(instructions);
    *memory.registers.get(&'a').unwrap()
}

fn run_and_get_register_a_part2(instructions: &[Instruction]) -> i32 {
    let mut memory = Memory::new_part2();
    memory.run_instructions(instructions);
    *memory.registers.get(&'a').unwrap()
}

fn parse_input(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            match split.next().unwrap() {
                "inc" => Instruction::Increment(split.next().unwrap().chars().next().unwrap()),
                "dec" => Instruction::Decrement(split.next().unwrap().chars().next().unwrap()),
                "cpy" => {
                    let register_int = parse_arg(split.next().unwrap());
                    let register = split.next().unwrap().chars().next().unwrap();
                    Instruction::Copy(register_int, register)
                }
                _ => {
                    let register_int = parse_arg(split.next().unwrap());
                    let value = split.next().unwrap().parse::<i32>().unwrap();
                    Instruction::JumpNotZero(register_int, value)
                }
            }
        })
        .collect()
}

fn parse_arg(arg: &str) -> RegisterInt {
    match arg.parse::<i32>() {
        Ok(n) => RegisterInt::Number(n),
        Err(_) => RegisterInt::Register(arg.chars().next().unwrap()),
    }
}

#[derive(Default)]
struct Memory {
    registers: HashMap<char, i32>,
    index: i32,
}

#[derive(PartialEq)]
enum RegisterInt {
    Register(char),
    Number(i32),
}

#[derive(PartialEq)]
enum Instruction {
    Increment(char),
    Decrement(char),
    Copy(RegisterInt, char),
    JumpNotZero(RegisterInt, i32),
}

impl Memory {
    fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert('a', 0);
        registers.insert('b', 0);
        registers.insert('c', 0);
        registers.insert('d', 0);
        Self {
            registers,
            index: 0,
        }
    }

    fn new_part2() -> Self {
        let mut registers = HashMap::new();
        registers.insert('a', 0);
        registers.insert('b', 0);
        registers.insert('c', 1);
        registers.insert('d', 0);
        Self {
            registers,
            index: 0,
        }
    }

    fn run_instructions(&mut self, instructions: &[Instruction]) {
        while (self.index as usize) < instructions.len() {
            self.run(instructions.get(self.index as usize).unwrap());
        }
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Increment(c) => self.increment(*c),
            Instruction::Decrement(c) => self.decrement(*c),
            Instruction::Copy(register_int, c) => self.copy(register_int, *c),
            Instruction::JumpNotZero(register_int, i) => self.jump_not_zero(register_int, *i),
        }
    }

    fn get_register(&self, register_int: &RegisterInt) -> i32 {
        match register_int {
            RegisterInt::Register(c) => *self.registers.get(c).unwrap(),
            RegisterInt::Number(i) => *i,
        }
    }

    fn copy(&mut self, register_int: &RegisterInt, destination: char) {
        let value_to_copy = self.get_register(&register_int);
        self.registers.insert(destination, value_to_copy);
        self.index += 1;
    }

    fn increment(&mut self, register: char) {
        self.registers.entry(register).and_modify(|v| *v += 1);
        self.index += 1;
    }

    fn decrement(&mut self, register: char) {
        self.registers.entry(register).and_modify(|v| *v -= 1);
        self.index += 1;
    }

    fn jump_not_zero(&mut self, register_int: &RegisterInt, distance: i32) {
        let check_value = self.get_register(register_int);
        if check_value != 0 {
            self.index += distance;
        } else {
            self.index += 1;
        }
    }
}
