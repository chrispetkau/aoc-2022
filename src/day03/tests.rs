use super::{input, solve_part1, solve_part2};

const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn part1() {
    assert_eq!(157, solve_part1(INPUT));
    assert_eq!(8185, solve_part1(input::INPUT));
}

#[test]
fn part2() {
    // assert_eq!(230, solve_part2(INPUT.0, &INPUT.1));
    // assert_eq!(3765399, solve_part2(input::INPUT.0, &input::INPUT.1));
}
