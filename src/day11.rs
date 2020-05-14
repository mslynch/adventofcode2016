use crate::solution::Solution;
use im::hashmap::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

/// Runs the solutions for day 11.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();

    let building = parse_input(&input);
    let state1 = BuildingState { floor: 0, building };

    let part1 = match state1.min_steps() {
        Ok(s) => s.to_string(),
        Err(e) => e,
    };

    let building_with_extras = state1.building.update_with(
        ElementPair {
            chip_floor: 0,
            rtg_floor: 0,
        },
        2,
        |old_count, _new_count| old_count + 2,
    );

    let state2 = BuildingState {
        floor: 0,
        building: building_with_extras,
    };

    let part2 = match state2.min_steps() {
        Ok(s) => s.to_string(),
        Err(e) => e,
    };

    Solution {
        title: "Radioisotope Thermoelectric Generators".to_string(),
        part1,
        part2,
    }
}

fn is_floor_out_of_bounds(floor: isize) -> bool {
    floor < 0 || floor > 3
}

type Building = HashMap<ElementPair, isize>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct BuildingState {
    floor: isize,
    building: Building,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct ElementPair {
    chip_floor: isize,
    rtg_floor: isize,
}

impl ElementPair {
    fn is_same_floor(&self) -> bool {
        self.chip_floor == self.rtg_floor
    }
}

impl BuildingState {
    fn min_steps(&self) -> Result<isize, String> {
        let mut visited_states = HashSet::new();
        visited_states.insert(self.clone());
        min_steps(&visited_states, &visited_states, 0)
    }

    fn is_valid(&self) -> bool {
        if is_floor_out_of_bounds(self.floor) {
            return false;
        }

        let is_item_floor_oob = self.building.iter().any(|(pair, _count)| {
            is_floor_out_of_bounds(pair.chip_floor) || is_floor_out_of_bounds(pair.rtg_floor)
        });
        if is_item_floor_oob {
            return false;
        }

        for (pair, _count) in self.building.iter() {
            if !pair.is_same_floor() {
                let is_pair_chip_fried = self
                    .building
                    .iter()
                    .any(|(other_pair, _count)| pair.chip_floor == other_pair.rtg_floor);
                if is_pair_chip_fried {
                    return false;
                }
            }
        }
        true
    }

    fn get_chip_pairs_on_current_floor(&self) -> Vec<ElementPair> {
        self.building
            .iter()
            .cloned()
            .filter(|(pair, _count)| pair.chip_floor == self.floor)
            .map(|(pair, _count)| pair)
            .collect()
    }

    fn get_rtg_pairs_on_current_floor(&self) -> Vec<ElementPair> {
        self.building
            .iter()
            .cloned()
            .filter(|(pair, _count)| pair.rtg_floor == self.floor)
            .map(|(pair, _count)| pair)
            .collect()
    }

    fn get_chip_down_states(&self, chip_pairs_on_current_floor: &[ElementPair]) -> Vec<Self> {
        chip_pairs_on_current_floor
            .iter()
            .map(|pair| {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor - 1,
                    rtg_floor: pair.rtg_floor,
                };
                BuildingState {
                    floor: self.floor - 1,
                    building: self.building.swap_out_pair(&pair, &new_pair),
                }
            })
            .collect()
    }

    fn get_chip_up_states(&self, chip_pairs_on_current_floor: &[ElementPair]) -> Vec<Self> {
        chip_pairs_on_current_floor
            .iter()
            .map(|pair| {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor + 1,
                    rtg_floor: pair.rtg_floor,
                };
                BuildingState {
                    floor: self.floor + 1,
                    building: self.building.swap_out_pair(&pair, &new_pair),
                }
            })
            .collect()
    }

    fn get_rtg_down_states(&self, rtg_pairs_on_current_floor: &[ElementPair]) -> Vec<Self> {
        rtg_pairs_on_current_floor
            .iter()
            .map(|pair| {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor,
                    rtg_floor: pair.rtg_floor - 1,
                };
                BuildingState {
                    floor: self.floor - 1,
                    building: self.building.swap_out_pair(&pair, &new_pair),
                }
            })
            .collect()
    }

    fn get_rtg_up_states(&self, rtg_pairs_on_current_floor: &[ElementPair]) -> Vec<Self> {
        rtg_pairs_on_current_floor
            .iter()
            .map(|pair| {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor,
                    rtg_floor: pair.rtg_floor + 1,
                };
                BuildingState {
                    floor: self.floor + 1,
                    building: self.building.swap_out_pair(&pair, &new_pair),
                }
            })
            .collect()
    }

    /// Gets possible states (pre-validation) from the current state.
    /// The elevator must carry at least one item, and may carry two of items of any type.
    fn get_possible_moves(&self) -> HashSet<BuildingState> {
        let chip_pairs_on_current_floor = self.get_chip_pairs_on_current_floor();
        let rtg_pairs_on_current_floor = self.get_rtg_pairs_on_current_floor();

        let one_chip_down_states = self.get_chip_down_states(&chip_pairs_on_current_floor);
        let one_chip_up_states = self.get_chip_up_states(&chip_pairs_on_current_floor);
        let one_rtg_down_states = self.get_rtg_down_states(&rtg_pairs_on_current_floor);
        let one_rtg_up_states = self.get_rtg_up_states(&rtg_pairs_on_current_floor);

        let second_item_moved_states: Vec<BuildingState> = one_chip_down_states
            .iter()
            .chain(one_chip_up_states.iter())
            .chain(one_rtg_down_states.iter())
            .chain(one_rtg_up_states.iter())
            .map(|state| BuildingState {
                floor: self.floor,
                building: state.building.clone(),
            })
            .flat_map(|state| {
                let chip_pairs_on_floor_after_move = state.get_chip_pairs_on_current_floor();
                let rtg_pairs_on_floor_after_move = state.get_rtg_pairs_on_current_floor();

                if state.floor < self.floor {
                    let and_chip_down_states =
                        state.get_chip_down_states(&chip_pairs_on_floor_after_move);
                    let and_rtg_down_states =
                        state.get_rtg_down_states(&rtg_pairs_on_floor_after_move);
                    and_chip_down_states
                        .into_iter()
                        .chain(and_rtg_down_states.into_iter())
                } else {
                    let and_chip_up_states =
                        state.get_chip_up_states(&chip_pairs_on_floor_after_move);
                    let and_rtg_up_states = state.get_rtg_up_states(&rtg_pairs_on_floor_after_move);
                    and_chip_up_states
                        .into_iter()
                        .chain(and_rtg_up_states.into_iter())
                }
            })
            .collect();

        one_chip_down_states
            .into_iter()
            .chain(one_chip_up_states.into_iter())
            .chain(one_rtg_down_states.into_iter())
            .chain(one_rtg_up_states.into_iter())
            .chain(second_item_moved_states.into_iter())
            .collect()
    }
}

trait BuildingTrait {
    fn is_finished(&self) -> bool;
    fn swap_out_pair(&self, pair_to_remove: &ElementPair, pair_to_insert: &ElementPair)
        -> Building;
}

impl BuildingTrait for Building {
    fn is_finished(&self) -> bool {
        self.iter()
            .all(|(pair, _count)| pair.chip_floor == 3 && pair.rtg_floor == 3)
    }

    fn swap_out_pair(&self, pair_to_remove: &ElementPair, pair_to_insert: &ElementPair) -> Self {
        self.alter(
            |existing| {
                let existing_count = existing.unwrap();
                match existing_count {
                    1 => None,
                    _ => Some(existing_count - 1),
                }
            },
            pair_to_remove.clone(),
        )
        .update_with(pair_to_insert.clone(), 1, |old_count, _new_count| {
            old_count + 1
        })
    }
}

fn parse_input(input: &[String]) -> Building {
    let item_floors_by_element =
        input
            .iter()
            .take(3)
            .enumerate()
            .fold(HashMap::new(), |acc, (floor_num, line)| {
                let mut split = line.split(' ');
                // consume "the nth floor contains a"
                for _ in 0..5 {
                    split.next();
                }
                let item_string = split.collect::<Vec<&str>>().join(" ");
                item_string
                    .chars()
                    .filter(|c| *c != ',' && *c != '.')
                    .collect::<String>()
                    .split(' ')
                    .filter(|item| !hashset! {"microchip", "and", "a", "generator"}.contains(item))
                    .fold(acc, |acc_2, item| {
                        let (element, value) = match item {
                            i if i.contains("-compatible") => (
                                item.split('-').next().unwrap().to_string(),
                                (Some(floor_num), None),
                            ),
                            _ => (item.to_string(), (None, Some(floor_num))),
                        };

                        acc_2.update_with(
                            element,
                            value,
                            |old_value, (new_chip_floor, new_rtg_floor)| match old_value {
                                (Some(chip_floor), None) => (Some(chip_floor), new_rtg_floor),
                                (None, Some(rtg_floor)) => (new_chip_floor, Some(rtg_floor)),
                                _ => panic!("found both or neither chip and rtg"),
                            },
                        )
                    })
            });

    item_floors_by_element.iter().fold(
        HashMap::new(),
        |acc, (_element, (chip_floor, rtg_floor))| {
            acc.update_with(
                ElementPair {
                    chip_floor: chip_floor.unwrap() as isize,
                    rtg_floor: rtg_floor.unwrap() as isize,
                },
                1,
                |old_count, _new_count| old_count + 1,
            )
        },
    )
}

fn min_steps(
    working_states: &HashSet<BuildingState>,
    visited_states: &HashSet<BuildingState>,
    steps_so_far: isize,
) -> Result<isize, String> {
    let new_states: HashSet<BuildingState> = working_states
        .iter()
        .flat_map(|current_state| {
            current_state
                .get_possible_moves()
                .into_iter()
                .filter(|state| !visited_states.contains(&state) && state.is_valid())
        })
        .collect();

    if new_states.iter().any(|state| state.building.is_finished()) {
        return Ok(steps_so_far + 1);
    }

    if new_states.is_empty() {
        return Err("could not find solution - unable to recurse".to_string());
    }

    let new_visited_states: HashSet<BuildingState> =
        visited_states.union(&new_states).cloned().collect();

    min_steps(&new_states, &new_visited_states, steps_so_far + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.".to_string(),
            "The second floor contains a hydrogen generator.".to_string(),
            "The third floor contains a lithium generator.".to_string(),
            "The fourth floor contains nothing relevant.".to_string()
        ];
        let parsed = parse_input(input.as_slice());

        assert_eq!(2, parsed.len());
        assert_eq!(
            parsed
                .get(&ElementPair {
                    chip_floor: 0,
                    rtg_floor: 1
                })
                .unwrap(),
            &1,
            "contains hydrogen pair"
        );

        assert_eq!(
            parsed
                .get(&ElementPair {
                    chip_floor: 0,
                    rtg_floor: 2
                })
                .unwrap(),
            &1,
            "contains lithium pair"
        );
    }

    #[test]
    fn invalid_building_chip_rtg_conflict_is_invalid() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: 0,
                rtg_floor: 1,
            },
            1,
        );
        building.insert(
            ElementPair {
                chip_floor: 1,
                rtg_floor: 0,
            },
            1,
        );
        let state = BuildingState { floor: 0, building };

        assert!(!state.is_valid());
    }

    #[test]
    fn valid_building_is_valid() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: 0,
                rtg_floor: 0,
            },
            1,
        );
        building.insert(
            ElementPair {
                chip_floor: 1,
                rtg_floor: 2,
            },
            1,
        );
        let state = BuildingState { floor: 0, building };

        assert!(state.is_valid());
    }

    #[test]
    fn below_bounds_building_is_invalid() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: -1,
                rtg_floor: 0,
            },
            1,
        );
        let state = BuildingState { floor: 0, building };
        assert!(!state.is_valid());
    }

    #[test]
    fn above_bounds_building_is_invalid() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: 4,
                rtg_floor: 0,
            },
            1,
        );
        let state = BuildingState { floor: 0, building };
        assert!(!state.is_valid());
    }

    #[test]
    fn below_bounds_floor_is_invalid() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: 0,
                rtg_floor: 0,
            },
            1,
        );
        let state = BuildingState {
            floor: -1,
            building,
        };
        assert!(!state.is_valid());
    }

    #[test]
    fn above_bounds_floor_is_invalid() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: 0,
                rtg_floor: 0,
            },
            1,
        );
        let state = BuildingState { floor: 4, building };
        assert!(!state.is_valid());
    }

    #[test]
    fn swap_out_pair_test() {
        let pair_1 = ElementPair {
            chip_floor: 0,
            rtg_floor: 0,
        };
        let pair_1_clone = pair_1.clone();
        let pair_2 = ElementPair {
            chip_floor: 1,
            rtg_floor: 1,
        };
        let building = HashMap::unit(pair_1, 1);

        let updated = building.swap_out_pair(&pair_1_clone, &pair_2);

        assert_eq!(1, updated.len());
        assert_eq!(1, *updated.get(&pair_2).unwrap());
    }

    #[test]
    fn finished_building() {
        let building = HashMap::new().update(
            ElementPair {
                chip_floor: 3,
                rtg_floor: 3,
            },
            2,
        );
        assert!(building.is_finished(), "building is finished");
    }

    #[test]
    fn unfinished_building() {
        let building = HashMap::new()
            .update(
                ElementPair {
                    chip_floor: 1,
                    rtg_floor: 1,
                },
                1,
            )
            .update(
                ElementPair {
                    chip_floor: 3,
                    rtg_floor: 3,
                },
                1,
            );
        assert!(!building.is_finished(), "building is unfinished");
    }

    #[test]
    fn get_chip_pairs_on_current_floor_test() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: 0,
                rtg_floor: 1,
            },
            1,
        );
        building.insert(
            ElementPair {
                chip_floor: 1,
                rtg_floor: 0,
            },
            1,
        );
        building.insert(
            ElementPair {
                chip_floor: 1,
                rtg_floor: 1,
            },
            1,
        );
        let state = BuildingState { floor: 0, building };
        let expected = vec![ElementPair {
            chip_floor: 0,
            rtg_floor: 1,
        }];
        let pairs = state.get_chip_pairs_on_current_floor();

        assert_eq!(expected, pairs);
    }

    #[test]
    fn get_rtg_pairs_on_current_floor_test() {
        let mut building = HashMap::new();
        building.insert(
            ElementPair {
                chip_floor: 0,
                rtg_floor: 1,
            },
            1,
        );
        building.insert(
            ElementPair {
                chip_floor: 1,
                rtg_floor: 0,
            },
            1,
        );
        building.insert(
            ElementPair {
                chip_floor: 1,
                rtg_floor: 1,
            },
            1,
        );
        let state = BuildingState { floor: 0, building };
        let expected = vec![ElementPair {
            chip_floor: 1,
            rtg_floor: 0,
        }];
        let pairs = state.get_rtg_pairs_on_current_floor();

        assert_eq!(expected, pairs);
    }

    #[test]
    fn get_chip_down_states_test() {
        let building = HashMap::new()
            .update(
                ElementPair {
                    chip_floor: 0,
                    rtg_floor: 1,
                },
                1,
            )
            .update(
                ElementPair {
                    chip_floor: 0,
                    rtg_floor: 2,
                },
                1,
            );
        let state = BuildingState { floor: 0, building };

        let chip_pairs_on_current_floor = vec![
            ElementPair {
                chip_floor: 0,
                rtg_floor: 1,
            },
            ElementPair {
                chip_floor: 0,
                rtg_floor: 2,
            },
        ];

        let expected_states = vec![
            BuildingState {
                floor: -1,
                building: {
                    HashMap::new()
                        .update(
                            ElementPair {
                                chip_floor: -1,
                                rtg_floor: 1,
                            },
                            1,
                        )
                        .update(
                            ElementPair {
                                chip_floor: 0,
                                rtg_floor: 2,
                            },
                            1,
                        )
                },
            },
            BuildingState {
                floor: -1,
                building: {
                    HashMap::new()
                        .update(
                            ElementPair {
                                chip_floor: 0,
                                rtg_floor: 1,
                            },
                            1,
                        )
                        .update(
                            ElementPair {
                                chip_floor: -1,
                                rtg_floor: 2,
                            },
                            1,
                        )
                },
            },
        ];
        let states = state.get_chip_down_states(&chip_pairs_on_current_floor);
        assert_eq!(expected_states, states);
    }
}
