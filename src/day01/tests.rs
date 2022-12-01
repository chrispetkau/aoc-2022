use super::{input, solve_for};

const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn part1() {
    assert_eq!(24000, solve_for(INPUT).unwrap().0);
    // assert_eq!(1557, solve_for(&input::INPUT).0);
}

#[test]
fn part2() {
    // assert_eq!(5, solve_for(&INPUT).1);
    // assert_eq!(1608, solve_for(&input::INPUT).1);
}
