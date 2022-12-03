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
    assert_eq!(70, solve_part2(INPUT));
    assert_eq!(2817, solve_part2(input::INPUT));
}
