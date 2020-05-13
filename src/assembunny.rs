use std::collections::HashMap;

#[derive(Default)]
pub struct Memory {
    pub registers: HashMap<char, i32>,
    index: i32,
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RegisterInt {
    Register(char),
    Number(i32),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Instruction {
    Increment(char),
    Decrement(char),
    Copy(RegisterInt, RegisterInt),
    JumpNotZero(RegisterInt, RegisterInt),
    Toggle(char),
    Output(char),
}

impl Instruction {
    fn toggle(&self) -> Self {
        match self {
            Self::Increment(c) => Self::Decrement(*c),
            Self::Decrement(c) => Self::Increment(*c),
            Self::JumpNotZero(ri1, ri2) => Self::Copy(*ri1, *ri2),
            Self::Copy(ri1, ri2) => Self::JumpNotZero(*ri1, *ri2),
            Self::Toggle(c) => Self::Increment(*c),
            Self::Output(c) => Self::Increment(*c),
        }
    }
}

impl Memory {
    pub fn new(registers: HashMap<char, i32>, instructions: Vec<Instruction>) -> Self {
        Self {
            registers,
            index: 0,
            instructions,
        }
    }

    pub fn run_instructions(&mut self) {
        self.for_each(|_| {});
    }

    pub fn provides_clock_signal(&mut self, iterations: usize) -> bool {
        let valid_clock = [0, 1].iter().cycle();
        let mut iter_count = 0;
        for (valid_output, self_output) in self.zip(valid_clock).take(iterations) {
            if valid_output != *self_output {
                return false;
            }
            iter_count += 1;
        }
        if iter_count != iterations {
            return false;
        }
        true
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

    fn output(&mut self, register: char) -> i32 {
        let value = self.get_register(RegisterInt::Register(register));
        self.index += 1;
        value
    }
}

impl Iterator for Memory {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        loop {
            let index = self.index as usize;
            if let Some(instruction) = self.instructions.get(index) {
                let output = match *instruction {
                    Instruction::Increment(c) => {
                        self.increment(c);
                        None
                    }
                    Instruction::Decrement(c) => {
                        self.decrement(c);
                        None
                    }
                    Instruction::Copy(register_int, destination) => {
                        self.copy(register_int, destination);
                        None
                    }
                    Instruction::JumpNotZero(register_int, distance) => {
                        self.jump_not_zero(register_int, distance);
                        None
                    }
                    Instruction::Toggle(c) => {
                        self.toggle(c);
                        None
                    }
                    Instruction::Output(c) => Some(self.output(c)),
                };
                if let Some(output_num) = output {
                    return Some(output_num);
                }
            } else {
                return None;
            }
        }
    }
}

pub fn parse_input(input: &[String]) -> Vec<Instruction> {
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
                }
                "jnz" => {
                    let register_int_1 = parse_arg(split.next().unwrap());
                    let register_int_2 = parse_arg(split.next().unwrap());
                    Instruction::JumpNotZero(register_int_1, register_int_2)
                }
                "out" => Instruction::Output(split.next().unwrap().chars().next().unwrap()),
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
