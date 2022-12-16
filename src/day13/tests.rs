use super::{input, solve_for};

const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[test]
fn part1() {
    assert_eq!(13, solve_for(INPUT).unwrap().0);
    assert_eq!(6272, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    // assert_eq!(PART2, solve_for(input::INPUT).unwrap().1);
}
