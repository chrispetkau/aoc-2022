use super::{input, solve_for};

const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[test]
fn part1() {
    assert_eq!(26, solve_for(INPUT, 10, 20).unwrap().0);
    assert_eq!(4_725_496, solve_for(input::INPUT, 2_000_000, 4_000_000).unwrap().0); // too high
}

#[test]
fn part2() {
    assert_eq!(56_000_011, solve_for(INPUT, 10, 20).unwrap().1);
    assert_eq!(12051287042458, solve_for(input::INPUT, 2_000_000, 4_000_000).unwrap().1);
}
