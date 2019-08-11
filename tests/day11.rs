extern crate adventofcode2016;
extern crate im;
use adventofcode2016::day11::{BuildingState, ElementPair};
use im::hashmap::HashMap;
use std::collections::HashSet;

#[test]
fn min_steps_test() {
    // The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    // The second floor contains a hydrogen generator.
    // The third floor contains a lithium generator.
    // The fourth floor contains nothing relevant.

    // F4 .  .  .  .  .
    // F3 .  .  .  LG .
    // F2 .  HG .  .  .
    // F1 E  .  HM .  LM

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
    let state = BuildingState {
        floor: 0,
        building: building,
    };

    assert_eq!(11, state.min_steps());
}
