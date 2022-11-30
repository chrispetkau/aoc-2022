use super::{brute_force_solution, input, span_solution};

pub(super) const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

/*
0,9 -> 5,9 h
8,0 -> 0,8
9,4 -> 3,4 h
2,2 -> 2,1 v
7,0 -> 7,4 v
6,4 -> 2,0
0,9 -> 2,9 h
3,4 -> 1,4 h
0,0 -> 8,8
5,5 -> 8,2
*/

#[test]
fn parse() {
    let lines = super::parse(INPUT).unwrap();
    assert_eq!(10, lines.len());
}

#[test]
fn part1_spans() {
    assert_eq!(5, span_solution::solve_for(&super::parse(INPUT).unwrap()).0);
    // TODO make spans solution work
    //assert_eq!(5774, span_solution::solve_for(&super::parse(input::INPUT).unwrap()).0);
}

#[test]
fn part2_spans() {
    // TODO make spans solution work
    //assert_eq!(12, span_solution::solve_for(&super::parse(INPUT).unwrap()).1);
    //assert_eq!(18423, span_solution::solve_for(&super::parse(input::INPUT).unwrap()).1);
}

#[test]
fn part1_brute_force() {
    assert_eq!(5, brute_force_solution::solve_for(&super::parse(INPUT).unwrap()).0);
    assert_eq!(
        5774,
        brute_force_solution::solve_for(&super::parse(input::INPUT).unwrap()).0
    );
}

#[test]
fn part2_brute_force() {
    assert_eq!(12, brute_force_solution::solve_for(&super::parse(INPUT).unwrap()).1);
    assert_eq!(
        18423,
        brute_force_solution::solve_for(&super::parse(input::INPUT).unwrap()).1
    );
}
