extern crate adventofcode2016;
extern crate im;

use im::hashmap::HashMap;
use adventofcode2016::day10::{Bot, State, OutputType, run_bots};

#[test]
fn run_bots_test() {
    let instructions = HashMap::new()
        .update(2, vec![Bot { id: 1, output_type: OutputType::Bot }, Bot { id: 0, output_type: OutputType::Bot }])
        .update(1, vec![Bot { id: 1, output_type: OutputType::Bin }, Bot { id: 0, output_type: OutputType::Bot }])
        .update(0, vec![Bot { id: 2, output_type: OutputType::Bin }, Bot { id: 0, output_type: OutputType::Bin }]);

    let state_bots = HashMap::new()
        .update(2, vec![2, 5])
        .update(1, vec![3]);


    let initial_state = State {
        bots: state_bots,
        bins: HashMap::new()
    };

    let expected_state = State {
        bots: HashMap::new()
            .update(2, vec![])
            .update(1, vec![])
            .update(0, vec![]),
        bins: HashMap::new()
            .update(0, 5)
            .update(1, 2)
            .update(2, 3)
    };

    let (actual_state, comparebot) = run_bots(instructions, initial_state, &[2, 5], None);
    
    assert_eq!(expected_state, actual_state);
    assert_eq!(2, comparebot.unwrap());
    
}
