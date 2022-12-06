use super::{input, solve_for};

#[test]
fn part1() {
    assert_eq!(7, solve_for("mjqjpqmgbljsphdztnvjfqwrcgsmlb").0);
    assert_eq!(5, solve_for("bvwbjplbgvbhsrlpgdmjqwftvncz").0);
    assert_eq!(6, solve_for("nppdvjthqldpwncqszvftbrmjlhg").0);
    assert_eq!(10, solve_for("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").0);
    assert_eq!(11, solve_for("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").0);
    assert_eq!(1300, solve_for(input::INPUT).0);
}

#[test]
fn part2() {
    // assert_eq!(1687617803407, solve_for(&input::INPUT, &[256])[0]);
}
