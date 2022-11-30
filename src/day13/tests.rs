use super::{input, solve_for};

const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

#[test]
fn part1() {
    assert_eq!(17, solve_for(INPUT).unwrap().0);
    assert_eq!(610, solve_for(input::INPUT).unwrap().0);
}

const PART2: &str = "###..####.####...##.#..#.###..####.####.
#..#....#.#.......#.#..#.#..#.#.......#.
#..#...#..###.....#.####.#..#.###....#..
###...#...#.......#.#..#.###..#.....#...
#....#....#....#..#.#..#.#.#..#....#....
#....####.#.....##..#..#.#..#.#....####.
";

#[test]
fn part2() {
    assert_eq!(PART2, solve_for(input::INPUT).unwrap().1);
}
