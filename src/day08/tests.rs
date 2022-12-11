use super::{input, solve_for};

const INPUT: &str =
"30373
25512
65332
33549
35390";

#[test]
fn part1() {
    assert_eq!(21, solve_for(INPUT).0);
    assert_eq!(1829, solve_for(input::INPUT).0);
}

#[test]
fn part2() {
    // assert_eq!(61229, solve_for(INPUT).1);
    // assert_eq!(1010460, solve_for(input::INPUT).1);
}
