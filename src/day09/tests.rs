use super::{input, solve_for};

const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn part1() {
    assert_eq!(15, solve_for(INPUT).0);
    assert_eq!(491, solve_for(input::INPUT).0);
}

#[test]
fn part2() {
    assert_eq!(1134, solve_for(INPUT).1);
    assert_eq!(1075536, solve_for(input::INPUT).1);
}
