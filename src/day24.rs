use itertools::Itertools;
use solution::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let cells_by_position = parse_input(&input);
    let location_data = get_location_data(&cells_by_position);
    let part1 = shortest_traversal(&location_data, false);
    let part2 = shortest_traversal(&location_data, true);

    Solution {
        title: "Safe Cracking".to_string(),
        part1: part1.to_string(),
        part2: part2.to_string(),
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position {
                row: self.row - 1,
                col: self.col,
            },
            Position {
                row: self.row + 1,
                col: self.col,
            },
            Position {
                row: self.row,
                col: self.col - 1,
            },
            Position {
                row: self.row,
                col: self.col + 1,
            },
        ]
    }
}

#[derive(Eq, Debug)]
struct Journey {
    from: Destination,
    to: Destination,
}

impl PartialEq for Journey {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
            || self.from == other.to && self.to == other.from
    }
}

impl Hash for Journey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.to.name < self.from.name {
            self.to.name.hash(state);
            self.from.name.hash(state);
        } else {
            self.from.name.hash(state);
            self.to.name.hash(state);
        }
    }
}

#[derive(PartialEq)]
enum Cell {
    Open,
    Wall,
    Location(char),
}

#[derive(Debug)]
struct LocationData {
    destinations: HashSet<Destination>,
    journey_distances: HashMap<Journey, usize>,
}

#[derive(Eq, Copy, Clone, Debug)]
struct Destination {
    name: char,
    position: Position,
}

impl Hash for Destination {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Destination {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn parse_input(input: &[String]) -> HashMap<Position, Cell> {
    input
        .iter()
        .enumerate()
        .fold(HashMap::new(), |acc, (row, line)| {
            line.chars().enumerate().fold(acc, |mut acc2, (col, cell)| {
                let position = Position { row, col };
                let cell = match cell {
                    '.' => Cell::Open,
                    '#' => Cell::Wall,
                    _ => Cell::Location(cell),
                };
                acc2.insert(position, cell);
                acc2
            })
        })
}

fn get_location_data(cells_by_position: &HashMap<Position, Cell>) -> LocationData {
    let destinations = cells_by_position
        .iter()
        .filter_map(|(position, cell)| match cell {
            Cell::Location(c) => Some(Destination {
                name: *c,
                position: *position,
            }),
            _ => None,
        })
        .collect::<HashSet<_>>();

    let journeys = destinations
        .iter()
        .flat_map(|d| {
            destinations
                .iter()
                .map(move |d2| Journey { from: *d, to: *d2 })
        })
        .filter(|journey| journey.from.name != journey.to.name)
        .collect::<HashSet<_>>();
    let journey_distances = journeys
        .into_iter()
        // .take(1)
        .map(|journey| {
            let distance = get_distance(cells_by_position, &journey);
            (journey, distance)
        })
        .collect();

    LocationData {
        destinations,
        journey_distances,
    }
}

fn get_distance(cells_by_position: &HashMap<Position, Cell>, journey: &Journey) -> usize {
    let mut visited = HashSet::new();
    let mut current = HashSet::new();
    current.insert(journey.from.position);
    visited.insert(journey.from.position);
    let mut distance = 0;
    while !visited.contains(&journey.to.position) {
        current = current
            .iter()
            .flat_map(|position| position.neighbors())
            .filter(|position| {
                *cells_by_position.get(position).unwrap() != Cell::Wall
                    && !visited.contains(position)
            })
            .collect::<HashSet<Position>>();
        for destination in current.iter() {
            visited.insert(*destination);
        }
        distance += 1;
    }
    distance
}

fn shortest_traversal(location_data: &LocationData, return_to_start: bool) -> usize {
    let starting_cell = location_data
        .destinations
        .iter()
        .find(|destination| destination.name == '0')
        .unwrap();
    let mut non_starting_destinations = location_data.destinations.clone();
    non_starting_destinations.remove(&starting_cell);
    non_starting_destinations
        .iter()
        .permutations(non_starting_destinations.len())
        .map(|path| {
            let path_iter: Box<dyn Iterator<Item = &Destination>> = if return_to_start {
                let iter = std::iter::once(starting_cell)
                    .chain(path.into_iter())
                    .chain(std::iter::once(starting_cell));
                Box::new(iter)
            } else {
                let iter = std::iter::once(starting_cell).chain(path.into_iter());
                Box::new(iter)
            };
            path_iter
                .tuple_windows()
                .map(|(dest1, dest2)| {
                    let journey = Journey {
                        from: *dest1,
                        to: *dest2,
                    };
                    location_data.journey_distances.get(&journey).unwrap()
                })
                .sum()
        })
        .min()
        .unwrap()
}
