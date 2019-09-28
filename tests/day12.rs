extern crate adventofcode2016;
extern crate im;
use adventofcode2016::day12::{run_and_get_register_a, Instruction, RegisterInt};

#[test]
fn min_steps_test() {
    let instructions = vec![
        Instruction::Copy(RegisterInt::Number(41), 'a'),
        Instruction::Increment('a'),
        Instruction::Increment('a'),
        Instruction::Decrement('a'),
        Instruction::JumpNotZero(RegisterInt::Register('a'), 2),
        Instruction::Decrement('a'),
    ];
    let final_register_a_value = run_and_get_register_a(&instructions);
    assert_eq!(42, final_register_a_value);
}
