use super::{input, solve_for};

const INPUT: [&str; 3] = [
    "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
];

#[test]
fn part1() {
    assert_eq!(10, solve_for(INPUT[0]).unwrap().0);
    assert_eq!(19, solve_for(INPUT[1]).unwrap().0);
    assert_eq!(226, solve_for(INPUT[2]).unwrap().0);
    assert_eq!(3713, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    assert_eq!(36, solve_for(INPUT[0]).unwrap().1);
    assert_eq!(103, solve_for(INPUT[1]).unwrap().1);
    assert_eq!(3509, solve_for(INPUT[2]).unwrap().1);
    assert_eq!(91292, solve_for(input::INPUT).unwrap().1);
}
