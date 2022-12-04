use super::{input, solve_for};

const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn part1() {
    assert_eq!(2, solve_for(INPUT).unwrap().0);
    assert_eq!(413, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    assert_eq!(4, solve_for(INPUT).unwrap().1);
    assert_eq!(806, solve_for(input::INPUT).unwrap().1);
}
