use super::{input, solve_for};

const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[test]
fn part1() {
    assert_eq!(26397, solve_for(INPUT).unwrap().0);
    assert_eq!(271245, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    assert_eq!(288957, solve_for(INPUT).unwrap().1);
    assert_eq!(1685293086, solve_for(input::INPUT).unwrap().1);
}
