use super::{input, solve_for};

const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn part1() {
    assert_eq!(1656, solve_for(INPUT).0);
    assert_eq!(1741, solve_for(input::INPUT).0);
}

#[test]
fn part2() {
    assert_eq!(195, solve_for(INPUT).1);
    assert_eq!(440, solve_for(input::INPUT).1);
}
