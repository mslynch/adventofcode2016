use solution::Solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let instructions = parse_input(&input);

    let part1 = run_and_get_register_a(instructions.clone(), 7).to_string();
    let part2 = run_and_get_register_a(instructions, 12).to_string();
    // let part2 = run_and_get_register_a_part2(&instructions).to_string();

    Solution {
        title: "Safe Cracking".to_string(),
        part1,
        part2,
    }
}

fn run_and_get_register_a(instructions: Vec<Instruction>, initial_a: i32) -> i32 {
    let mut memory = Memory::new(instructions, initial_a);
    memory.run_instructions();
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
                    let register_int_1 = parse_arg(split.next().unwrap());
                    let register_int_2 = parse_arg(split.next().unwrap());
                    Instruction::Copy(register_int_1, register_int_2)
                },
                "jnz" => {
                    let register_int_1 = parse_arg(split.next().unwrap());
                    let register_int_2 = parse_arg(split.next().unwrap());
                    Instruction::JumpNotZero(register_int_1, register_int_2)
                },
                _ => Instruction::Toggle(split.next().unwrap().chars().next().unwrap()),
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
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum RegisterInt {
    Register(char),
    Number(i32),
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Instruction {
    Increment(char),
    Decrement(char),
    Copy(RegisterInt, RegisterInt),
    JumpNotZero(RegisterInt, RegisterInt),
    Toggle(char),
}

impl Instruction {
    fn toggle(&self) -> Self {
        match self {
            Self::Increment(c) => Self::Decrement(*c),
            Self::Decrement(c) => Self::Increment(*c),
            Self::JumpNotZero(ri1, ri2) => Self::Copy(*ri1, *ri2),
            Self::Copy(ri1, ri2) => Self::JumpNotZero(*ri1, *ri2),
            Self::Toggle(c) => Self::Increment(*c),
        }
    }
}

impl Memory {
    fn new(instructions: Vec<Instruction>, initial_a: i32) -> Self {
        let mut registers = HashMap::new();
        registers.insert('a', initial_a);
        registers.insert('b', 0);
        registers.insert('c', 0);
        registers.insert('d', 0);
        Self {
            registers,
            index: 0,
            instructions,
        }
    }

    fn run_instructions(&mut self) {
        while (self.index as usize) < self.instructions.len() {
            let next_index = self.index as usize;
            let instruction = *self.instructions.get(next_index).unwrap();
            
            match instruction {
                Instruction::Increment(c) => self.increment(c),
                Instruction::Decrement(c) => self.decrement(c),
                Instruction::Copy(register_int, destination) => self.copy(register_int, destination),
                Instruction::JumpNotZero(register_int, distance) => self.jump_not_zero(register_int, distance),
                Instruction::Toggle(c) => self.toggle(c),
            }
        }
    }

    fn get_register(&self, register_int: RegisterInt) -> i32 {
        match register_int {
            RegisterInt::Register(c) => *self.registers.get(&c).unwrap(),
            RegisterInt::Number(i) => i,
        }
    }

    fn copy(&mut self, register_int: RegisterInt, destination: RegisterInt) {
        if let RegisterInt::Register(c) = destination {
            let value_to_copy = self.get_register(register_int);
            self.registers.insert(c, value_to_copy);
        }
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

    fn jump_not_zero(&mut self, register_int: RegisterInt, distance: RegisterInt) {
        let check_value = self.get_register(register_int);
        if check_value != 0 {
            self.index += self.get_register(distance);
        } else {
            self.index += 1;
        }
    }

    fn toggle(&mut self, register: char) {
        let value = self.get_register(RegisterInt::Register(register));
        let toggle_index = self.index as usize + value as usize;
        if let Some(instruction_to_toggle) = self.instructions.get(toggle_index) {
            self.instructions[toggle_index] = instruction_to_toggle.toggle();    
        }
        self.index += 1;
    }
}
