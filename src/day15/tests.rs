use super::{input, solve_for};

const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[test]
fn part1() {
    assert_eq!(40, solve_for(INPUT).unwrap().0);
    assert_eq!(685, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    assert_eq!(315, solve_for(INPUT).unwrap().1);
    assert_eq!(2995, solve_for(input::INPUT).unwrap().1);
}
