use super::{input, solve_for};

const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[test]
fn part1() {
    assert_eq!(24, solve_for(INPUT).unwrap().0);
    assert_eq!(698, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    assert_eq!(93, solve_for(INPUT).unwrap().1);
    assert_eq!(28594, solve_for(input::INPUT).unwrap().1);
}
