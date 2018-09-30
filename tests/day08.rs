extern crate adventofcode2016;

use adventofcode2016::day08::run_instructions;

#[test]
fn lit_after_instructions_test() {
  let instructions = [
    "rect 3x2".to_string(),
    "rotate column x=1 by 1".to_string(),
    "rotate row y=0 by 4".to_string(),
    "rotate column x=1 by 1".to_string(),
  ];
  let (lit, _screen) = run_instructions(&instructions);
  assert_eq!(lit, 6);
}
