use super::{input, solve_for};

const INPUT: [usize; 5] = [3, 4, 3, 1, 2];

#[test]
fn part1() {
    assert_eq!(26, solve_for(&INPUT, &[18])[0]);
    assert_eq!(5934, solve_for(&INPUT, &[80])[0]);
    assert_eq!(374927, solve_for(&input::INPUT, &[80])[0]);
}

#[test]
fn part2() {
    assert_eq!(1687617803407, solve_for(&input::INPUT, &[256])[0]);
}
