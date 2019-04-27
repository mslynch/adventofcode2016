use std::collections::HashSet;
use im::hashmap::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use std::collections::LinkedList;

/// Runs the solutions for day 11.
pub fn run(filename: Option<&str>) {
    println!("Day 11: Radioisotope Thermoelectric Generators");
    let file = File::open(filename.unwrap_or("data/day11.txt")).expect("file not found");
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let building = parse_input(&input);
    
    // let steps = min_steps(&building, &HashSet::new(), 0).unwrap();
    // println!("part 1: {}", steps); // not 14
}

pub type Building = HashMap<ElementPair, usize>;

pub struct BuildingState {
    pub floor: usize,
    pub building: Building
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ItemType {
    Chip,
    RTG,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Item {
    pub element: String,
    pub kind: ItemType,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ElementPair {
    pub chip_floor: usize,
    pub rtg_floor: usize,
}

impl ElementPair {
    fn same_floor(&self) -> bool {
        self.chip_floor == self.rtg_floor
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
                            |old_value, (new_chip_floor, new_rtg_floor)| {
                                match old_value {
                                    (Some(chip_floor), None) => (Some(chip_floor), new_rtg_floor),
                                    (None, Some(rtg_floor)) => (new_chip_floor, Some(rtg_floor)),
                                    _ => panic!("found both or neither chip and rtg"),
                                }
                            },
                        )
                    })
            });

    item_floors_by_element
        .iter()
        .fold(HashMap::new(), |acc, (_element, (chip_floor, rtg_floor))| {
            println!("{:?}", acc);
            acc.update_with(
                ElementPair {
                    chip_floor: chip_floor.unwrap(),
                    rtg_floor: rtg_floor.unwrap(),
                },
                1,
                |old_count, _new_count| old_count + 1,
            )
        })
}

fn is_valid_building(building: &Building) -> bool {
    for (element_pair, _count) in building.iter() {
        if !element_pair.same_floor() {
            if building.iter().any(|(other_element_pair, _count)| {
                element_pair != other_element_pair && !other_element_pair.same_floor()
            }) {
                return false;
            }
        }
    }
    true
}

fn is_finished(building: &Building) -> bool {
    building
        .iter()
        .all(|(pair, _count)| pair.chip_floor == 3 && pair.rtg_floor == 3)
}

fn swap_out_pair(
    building: &Building,
    remove_pair: &ElementPair,
    insert_pair: &ElementPair,
) -> Building {
    building
        .alter(
            |existing| {
                let existing_count = existing.unwrap();
                match existing_count {
                    1 => None,
                    _ => Some(existing_count - 1),
                }
            },
            remove_pair.clone(),
        )
        .update_with(insert_pair.clone(), 1, |old_count, _new_count| {
            old_count + 1
        })
}

fn valid_moves(building: &Building, floor: usize) -> HashSet<BuildingState> {
    // going up
    let mut moves = HashSet::new();
    if floor < 3 {
        building.iter().filter(||)
    }

    unimplemented!()
}

// TODO: create a visit building buffer. start off with just the base building, then add the new_buildings into it. recurse by passing buffer in
pub fn min_steps(buildings: &mut LinkedList<Building>, visited_states: &HashSet<Building>, steps_so_far: usize) -> Option<usize> {
    println!();
    println!("steps_so_far: {}", steps_so_far);
    // let current_building = buildings.iter().cloned().find(|building| !visited_states.contains(building)).unwrap();

    //  TODO refactor
    let mut current_building = HashMap::new();
    let mut found = false;
    while !found {
        current_building = buildings.pop_front().unwrap();
        found = !visited_states.contains(&current_building);
    } 

    println!("current building: {:?}", current_building);
    let new_buildings: HashSet<Building> = current_building
        .keys()
        .flat_map(|pair| {
            let mut possible_buildings = Vec::new();
            if pair.chip_floor > 0 {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor - 1,
                    rtg_floor: pair.rtg_floor,
                };
                possible_buildings.push(swap_out_pair(&current_building, &pair, &new_pair));
            }
            if pair.chip_floor < 3 {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor + 1,
                    rtg_floor: pair.rtg_floor,
                };
                possible_buildings.push(swap_out_pair(&current_building, &pair, &new_pair));
            }
            if pair.rtg_floor > 0 {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor,
                    rtg_floor: pair.rtg_floor - 1,
                };
                possible_buildings.push(swap_out_pair(&current_building, &pair, &new_pair));
            }
            if pair.rtg_floor < 3 {
                let new_pair = ElementPair {
                    chip_floor: pair.chip_floor,
                    rtg_floor: pair.rtg_floor + 1,
                };
                possible_buildings.push(swap_out_pair(&current_building, &pair, &new_pair));
            }
            possible_buildings.into_iter()
        })
        .filter(|building| !visited_states.contains(&building) && is_valid_building(building))
        .collect();

    println!("new_buildings len: {}", new_buildings.len());

    if new_buildings.iter().all(|building| is_finished(building)) {
        return Some(steps_so_far);
    }

    for building in new_buildings.clone() {
        buildings.push_front(building);
    }

    let new_visited_states = visited_states.union(&new_buildings).cloned().collect();

    min_steps(buildings, &new_visited_states, steps_so_far + 1)
    // new_buildings
    //     .iter()
    //     .map(|building| min_steps(buildings, &new_visited_states, steps_so_far + 1))
    //     .next()
    //     .unwrap_or(None)
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
    fn valid_building_test() {
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

        assert!(!is_valid_building(&building));
    }

    #[test]
    fn invalid_building_test() {
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

        assert!(is_valid_building(&building));
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

        let updated = swap_out_pair(&building, &pair_1_clone, &pair_2);

        assert_eq!(1, updated.len());
        assert_eq!(1, *updated.get(&pair_2).unwrap());
    }

    #[test]
    fn finished_building() {
        let building = HashMap::new()
            .update( ElementPair {chip_floor: 3, rtg_floor:  3 }, 2);
        assert!(is_finished(&building), "building is finished");
    }


    #[test]
    fn unfinished_building() {
        let building = HashMap::new()
            .update( ElementPair {chip_floor: 1, rtg_floor:  1 }, 1)
            .update( ElementPair {chip_floor: 3, rtg_floor:  3 }, 1);
        assert!(!is_finished(&building), "building is finished");
    }

}
