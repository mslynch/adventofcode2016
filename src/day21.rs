use solution::Solution;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(file: &mut File) -> Solution {
    run_with_string(file, "abcdefgh", "fbgdceah")
}

pub fn run_with_string(
    file: &mut File,
    string_to_scramble: &str,
    string_to_unscramble: &str,
) -> Solution {
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let scrambled = scramble(string_to_scramble, &input, &Operation::Scramble);
    let unscrambled = scramble(string_to_unscramble, &input, &Operation::Unscramble);

    Solution {
        title: "Scrambled Letters and Hash".to_string(),
        part1: scrambled,
        part2: unscrambled,
    }
}

enum Operation {
    Scramble,
    Unscramble,
}

fn scramble(prescrambled: &str, input: &[String], operation: &Operation) -> String {
    let rotate_steps = match operation {
        Operation::Scramble => rotate_steps,
        Operation::Unscramble => invert_rotate_steps,
    };
    let rotate_on_letter = match operation {
        Operation::Scramble => rotate_on_letter,
        Operation::Unscramble => invert_rotate_on_letter,
    };
    let move_position = match operation {
        Operation::Scramble => move_position,
        Operation::Unscramble => invert_move_position,
    };

    let instruction_iter: Box<dyn Iterator<Item = &String>> = match operation {
        Operation::Scramble => Box::new(input.iter()),
        Operation::Unscramble => Box::new(input.iter().rev()),
    };

    instruction_iter.fold(prescrambled.to_string(), |acc, instruction| {
        println!("running with {}, {}", acc, instruction);
        let mut split = instruction.split(' ');
        let first = split.next().unwrap();
        match first {
            "swap" => {
                let position_or_letter = split.next().unwrap();
                let x = split.next().unwrap();
                split.next();
                split.next();
                let y = split.next().unwrap();
                match position_or_letter {
                    "position" => swap_position(&acc, x.parse().unwrap(), y.parse().unwrap()),
                    _letter => swap_letter(&acc, x.parse().unwrap(), y.parse().unwrap()),
                }
            }
            "rotate" => {
                let direction_or_based = split.next().unwrap();
                match direction_or_based {
                    "based" => {
                        split.next();
                        split.next();
                        split.next();
                        split.next();
                        let letter = split.next().unwrap().chars().next().unwrap();
                        rotate_on_letter(&acc, letter)
                    }
                    direction => {
                        let steps = split.next().unwrap().parse::<usize>().unwrap();
                        rotate_steps(&acc, &direction, steps)
                    }
                }
            }
            "reverse" => {
                split.next();
                let x = split.next().unwrap().parse::<usize>().unwrap();
                split.next();
                let y = split.next().unwrap().parse::<usize>().unwrap();
                reverse(&acc, x, y)
            }
            _move => {
                split.next();
                let x = split.next().unwrap().parse::<usize>().unwrap();
                split.next();
                split.next();
                let y = split.next().unwrap().parse::<usize>().unwrap();
                move_position(&acc, x, y)
            }
        }
    })
}

fn swap_position(input: &str, x: usize, y: usize) -> String {
    let x_char = input.chars().nth(x).unwrap();
    let y_char = input.chars().nth(y).unwrap();
    let mut char_vec: Vec<char> = input.chars().collect();
    char_vec[x] = y_char;
    char_vec[y] = x_char;
    char_vec.iter().collect()
}

fn swap_letter(input: &str, x: char, y: char) -> String {
    let x_index = input.chars().position(|c| c == x).unwrap();
    let y_index = input.chars().position(|c| c == y).unwrap();
    swap_position(&input, x_index, y_index)
}

fn rotate_steps(input: &str, direction: &str, steps: usize) -> String {
    let mod_shift = steps % input.len();
    let shift_by = match direction {
        "left" => mod_shift,
        _right => input.len() - mod_shift,
    };
    input
        .chars()
        .cycle()
        .skip(shift_by)
        .take(input.len())
        .collect()
}

fn invert_rotate_steps(input: &str, direction: &str, steps: usize) -> String {
    rotate_steps(&input, &direction, input.len() - (steps % input.len()))
}

fn rotate_on_letter(input: &str, letter: char) -> String {
    let index_of_letter = input.chars().position(|c| c == letter).unwrap();
    let rotation_amount = if index_of_letter >= 4 {
        index_of_letter + 2
    } else {
        index_of_letter + 1
    };
    rotate_steps(input, "right", rotation_amount)
}

fn invert_rotate_on_letter(input: &str, letter: char) -> String {
    (1..)
        .map(|i| rotate_steps(&input, "left", i))
        .find(|s| rotate_on_letter(&s, letter) == input)
        .unwrap()
}

fn reverse(input: &str, x: usize, y: usize) -> String {
    let before_x = &input[..x];
    let reversed = &input[x..=y].chars().rev().collect::<String>();
    let after_y = &input[(y + 1)..];
    format!("{}{}{}", before_x, reversed, after_y)
}

fn move_position(input: &str, x: usize, y: usize) -> String {
    let mut as_vec = input.chars().collect::<Vec<char>>();
    let c = as_vec.remove(x);
    as_vec.insert(y, c);
    as_vec.iter().collect()
}

fn invert_move_position(input: &str, x: usize, y: usize) -> String {
    move_position(&input, y, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_position_test() {
        assert_eq!("acbd".to_string(), swap_position("abcd", 1, 2));
    }

    #[test]
    fn swap_letter_test() {
        assert_eq!("acbd".to_string(), swap_letter("abcd", 'b', 'c'));
    }

    #[test]
    fn rotate_steps_test_1() {
        assert_eq!("dabc".to_string(), rotate_steps("abcd", "right", 1));
    }

    #[test]
    fn invert_steps_test_1() {
        let result = invert_rotate_steps(&rotate_steps("abcd", "right", 1), "right", 1);
        assert_eq!("abcd".to_string(), result);
    }

    #[test]
    fn rotate_steps_test_2() {
        assert_eq!("bcda".to_string(), rotate_steps("abcd", "left", 1));
    }

    #[test]
    fn invert_rotate_steps_test_2() {
        let result = invert_rotate_steps(&rotate_steps("abcd", "left", 1), "left", 1);
        assert_eq!("abcd".to_string(), result);
    }

    #[test]
    fn rotate_on_letter_test_1() {
        assert_eq!("dabc", rotate_on_letter("abcd", 'a'));
    }

    #[test]
    fn invert_rotate_on_letter_test_1() {
        let result = invert_rotate_on_letter(&rotate_on_letter("abcd", 'a'), 'a');
        assert_eq!("abcd", result);
    }

    #[test]
    fn rotate_on_letter_test_2() {
        assert_eq!("cdab", rotate_on_letter("abcd", 'b'));
    }

    #[test]
    fn invert_rotate_on_letter_test_2() {
        let result = invert_rotate_on_letter(&rotate_on_letter("abcd", 'b'), 'b');
        assert_eq!("abcd", result);
    }

    #[test]
    fn rotate_on_letter_test_3() {
        assert_eq!("abcdef", rotate_on_letter("abcdef", 'e'));
    }

    #[test]
    fn invert_rotate_on_letter_test_3() {
        let result = invert_rotate_on_letter(&rotate_on_letter("abcdef", 'e'), 'e');
        assert_eq!("abcdef", result);
    }

    #[test]
    fn reverse_test() {
        assert_eq!("acbd", reverse("abcd", 1, 2));
    }

    #[test]
    fn move_position_test() {
        assert_eq!("acdebf", move_position("abcdef", 1, 4));
    }

    #[test]
    fn invert_move_position_test() {
        let result = invert_move_position(&move_position("abcdef", 1, 4), 1, 4);
        assert_eq!("abcdef", result);
    }
}
