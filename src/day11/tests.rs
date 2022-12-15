use super::{input, solve_for};

const INPUT: &str = "Monkey 0:
Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[test]
fn part1() {
    assert_eq!(10605, solve_for(INPUT).unwrap().0);
    assert_eq!(76728, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    assert_eq!(2713310158, solve_for(INPUT).unwrap().1);
    assert_eq!(21553910156, solve_for(input::INPUT).unwrap().1);
}
