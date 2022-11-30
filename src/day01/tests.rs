use super::{input, solve_for};

const INPUT: [usize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

#[test]
fn part1() {
    assert_eq!(7, solve_for(&INPUT).0);
    assert_eq!(1557, solve_for(&input::INPUT).0);
}

#[test]
fn part2() {
    assert_eq!(5, solve_for(&INPUT).1);
    assert_eq!(1608, solve_for(&input::INPUT).1);
}
