use crate::day05::{parse, solve_for};

use super::input;

pub(super) const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn part1() {
    let (mut example_stacks, example_steps) = parse(INPUT).unwrap();
    assert_eq!("CMZ", solve_for(&mut example_stacks, &example_steps).0);

    let (mut stacks, steps) = parse(input::INPUT).unwrap();
    assert_eq!("QNNTGTPFN", solve_for(&mut stacks, &steps).0);
}

#[test]
fn part2() {
    let (mut example_stacks, example_steps) = parse(INPUT).unwrap();
    assert_eq!("MCD", solve_for(&mut example_stacks, &example_steps).1);

    let (mut stacks, steps) = parse(input::INPUT).unwrap();
    assert_eq!("GGNPJBTTR", solve_for(&mut stacks, &steps).1);
}
