use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter;

/// Runs the problems for day 1.
pub fn run(filename: Option<&str>) {
    println!("Day 1: No Time for a Taxicab");
    let mut file = File::open(filename.unwrap_or("data/day01.txt")).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let (ending_distance, revisited_distance) = blocks_away(&contents);
    println!("part 1: {}", ending_distance);
    println!("part 2: {}", revisited_distance.unwrap());
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}

struct WalkState {
    position: Position,
    direction: Direction,
}

struct Instruction {
    relative_direction: char,
    distance: isize,
}

impl Direction {
    /// Calculates the new direction after turning.
    fn turn(&self, relative_direction: char) -> Direction {
        match relative_direction {
            'L' => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            _ => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
        }
    }
}

impl WalkState {
    /// Calculates the position and direction after following an instruction.
    fn walk(&self, instruction: &Instruction) -> WalkState {
        let new_direction = self.direction.turn(instruction.relative_direction);
        let new_position = match new_direction {
            Direction::North => Position {
                x: self.position.x,
                y: self.position.y + instruction.distance,
            },
            Direction::South => Position {
                x: self.position.x,
                y: self.position.y - instruction.distance,
            },
            Direction::East => Position {
                x: self.position.x + instruction.distance,
                y: self.position.y,
            },
            Direction::West => Position {
                x: self.position.x - instruction.distance,
                y: self.position.y,
            },
        };
        WalkState {
            position: new_position,
            direction: new_direction,
        }
    }
}

impl Position {
    /// The Manhattan distance of a position to the origin.
    fn absolute_blocks(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    /// The range from this position to the given position, not including this position.
    fn range_to(&self, end: &Position) -> Vec<Position> {
        let x_updater = |x| Position { x, y: self.y };
        let y_updater = |y| Position { x: self.x, y };

        match self.x.cmp(&end.x) {
            Ordering::Less => (self.x + 1..=end.x).map(x_updater).collect(),
            Ordering::Greater => (end.x..self.x).map(x_updater).rev().collect(),
            _ => match self.y.cmp(&end.y) {
                Ordering::Less => (self.y + 1..=end.y).map(y_updater).collect(),
                _ => (end.y..self.y).map(y_updater).rev().collect(),
            },
        }
    }
}

/// Returns a tuple of the ending distance and the first revisited distance.
pub fn blocks_away(input: &str) -> (isize, Option<isize>) {
    let initial_state = WalkState {
        position: Position { x: 0, y: 0 },
        direction: Direction::North,
    };

    let final_walk_result = input
        .split(", ")
        .map(|i| Instruction {
            relative_direction: i.chars().next().unwrap(),
            distance: i[1..].parse().unwrap(),
        }).scan(initial_state, |last_state, instruction| {
            let after_walk = last_state.walk(&instruction);
            let positions = last_state.position.range_to(&after_walk.position);

            *last_state = WalkState {
                position: positions.iter().last().unwrap().clone(),
                direction: after_walk.direction,
            };

            Some(positions)
        });

    let walked_positions: Vec<Position> = final_walk_result
        .flat_map(|walk_result| walk_result)
        .collect();

    let mut visited = HashSet::new();
    let mut first_visited_position = None;

    for position in iter::once(&Position { x: 0, y: 0 }).chain(walked_positions.iter()) {
        if !visited.insert(position) {
            first_visited_position = Some(position);
            break;
        }
    }

    let final_position = walked_positions.iter().last().unwrap();
    let ending_distance = final_position.absolute_blocks();

    let revisited_distance = first_visited_position.and_then(|pos| Some(pos.absolute_blocks()));

    (ending_distance, revisited_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_range_x_increasing() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 3, y: 0 };
        let expected = vec![
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 3, y: 0 },
        ];
        assert_eq!(expected, start.range_to(&end));
    }

    #[test]
    fn position_range_x_decreasing() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: -3, y: 0 };
        let expected = vec![
            Position { x: -1, y: 0 },
            Position { x: -2, y: 0 },
            Position { x: -3, y: 0 },
        ];
        assert_eq!(expected, start.range_to(&end));
    }

    #[test]
    fn position_range_y_increasing() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 0, y: 3 };
        let expected = vec![
            Position { x: 0, y: 1 },
            Position { x: 0, y: 2 },
            Position { x: 0, y: 3 },
        ];
        assert_eq!(expected, start.range_to(&end));
    }

    #[test]
    fn position_range_y_decreasing() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 0, y: -3 };
        let expected = vec![
            Position { x: 0, y: -1 },
            Position { x: 0, y: -2 },
            Position { x: 0, y: -3 },
        ];
        assert_eq!(expected, start.range_to(&end));
    }
}
