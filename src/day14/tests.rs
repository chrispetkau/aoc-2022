use super::{input, solve_for};

const INPUT: (&str, &str) = (
    "NNCB",
    "CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
);

#[test]
fn part1() {
    assert_eq!(1588, solve_for(INPUT).0);
    assert_eq!(2937, solve_for(input::INPUT).0);
}

#[test]
fn part2() {
    assert_eq!(2188189693529, solve_for(INPUT).1);
    assert_eq!(3390034818249, solve_for(input::INPUT).1);
}
