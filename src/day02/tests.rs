use super::{input, solve_part1, solve_part2, Direction};

const INPUT: [(Direction, usize); 6] = [
    (Direction::Forward, 5),
    (Direction::Down, 5),
    (Direction::Forward, 8),
    (Direction::Up, 3),
    (Direction::Down, 8),
    (Direction::Forward, 2),
];

#[test]
fn part1() {
    assert_eq!(150, solve_part1(&INPUT));
    assert_eq!(1804520, solve_part1(&input::INPUT));
}

#[test]
fn part2() {
    assert_eq!(900, solve_part2(&INPUT));
    assert_eq!(1971095320, solve_part2(&input::INPUT));
}
