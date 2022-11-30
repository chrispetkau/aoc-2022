use super::{input, solve_for};

const INPUT: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

#[test]
fn part1() {
    assert_eq!(37, solve_for(&INPUT).0);
    assert_eq!(339321, solve_for(&input::INPUT).0);
}

#[test]
fn part2() {
    assert_eq!(168, solve_for(&INPUT).1);
    assert_eq!(95476244, solve_for(&input::INPUT).1);
}
