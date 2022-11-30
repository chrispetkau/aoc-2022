use super::{input, solve_part1, solve_part2};

mod reference_solution;

const INPUT: (usize, [usize; 12]) = (
    5,
    [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ],
);

#[test]
fn part1() {
    assert_eq!(198, solve_part1(INPUT.0, &INPUT.1));
    assert_eq!(3549854, solve_part1(input::INPUT.0, &input::INPUT.1));
}

#[test]
fn part2_reference() {
    assert_eq!(230, reference_solution::solve_part2(INPUT.0, &INPUT.1));
    assert_eq!(
        3765399,
        reference_solution::solve_part2(input::INPUT.0, &input::INPUT.1)
    );
}

#[test]
fn part2() {
    assert_eq!(230, solve_part2(INPUT.0, &INPUT.1));
    assert_eq!(3765399, solve_part2(input::INPUT.0, &input::INPUT.1));
}
