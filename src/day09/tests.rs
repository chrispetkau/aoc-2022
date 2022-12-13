use super::{input, solve_for};

const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn part1() {
    assert_eq!(13, solve_for(INPUT).0);
    assert_eq!(5981, solve_for(input::INPUT).0);
}

#[test]
fn part2() {
    // assert_eq!(1134, solve_for(INPUT).1);
    // assert_eq!(1075536, solve_for(input::INPUT).1);
}
