use super::{input, solve_for};

const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[test]
fn part1() {
    assert_eq!(31, solve_for(INPUT).unwrap().0);
    assert_eq!(408, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    // assert_eq!(36, solve_for(INPUT).unwrap().1);
    // assert_eq!(91292, solve_for(input::INPUT).unwrap().1);
}
