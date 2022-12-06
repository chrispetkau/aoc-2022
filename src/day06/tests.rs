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
    assert_eq!(19, solve_for("mjqjpqmgbljsphdztnvjfqwrcgsmlb").1);
    assert_eq!(23, solve_for("bvwbjplbgvbhsrlpgdmjqwftvncz").1);
    assert_eq!(23, solve_for("nppdvjthqldpwncqszvftbrmjlhg").1);
    assert_eq!(29, solve_for("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").1);
    assert_eq!(26, solve_for("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").1);
    assert_eq!(3986, solve_for(input::INPUT).1);
}
