use super::{input, solve_part1, solve_part2};

const INPUT: &str= "A Y
B X
C Z";

#[test]
fn part1() {
    assert_eq!(15, solve_part1(INPUT));
    // assert_eq!(1804520, solve_part1(input::INPUT));
}

#[test]
fn part2() {
    assert_eq!(900, solve_part2(INPUT));
    assert_eq!(1971095320, solve_part2(input::INPUT));
}
