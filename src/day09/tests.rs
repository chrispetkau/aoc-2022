use super::{input, solve_for};

const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn part1() {
    assert_eq!(13, solve_for(INPUT1).0);
    assert_eq!(5981, solve_for(input::INPUT).0);
}

const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[test]
fn part2() {
    assert_eq!(36, solve_for(INPUT2).1);
    assert_eq!(2352, solve_for(input::INPUT).1);
}
