extern crate adventofcode2016;
extern crate im;
use std::collections::HashSet;
use std::collections::LinkedList;
use im::hashmap::HashMap;
use adventofcode2016::day11::{min_steps, ElementPair};


#[test]
fn min_steps_test() {
    let building = HashMap::new()
        .update(ElementPair { chip_floor: 0, rtg_floor: 1}, 1)
        .update(ElementPair { chip_floor: 0, rtg_floor: 2}, 1);
    let mut buildings = LinkedList::new();
    buildings.push_front(building);

    assert_eq!(Some(11), min_steps(&mut buildings, &HashSet::new(), 0));
}
