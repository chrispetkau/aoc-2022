use super::{input, solve_for};

#[test]
fn part1() {
    assert_eq!(16, solve_for("8A004A801A8002F478").unwrap().0);
    assert_eq!(12, solve_for("620080001611562C8802118E34").unwrap().0);
    assert_eq!(23, solve_for("C0015000016115A2E0802F182340").unwrap().0);
    assert_eq!(31, solve_for("A0016C880162017C3686B18A3D4780").unwrap().0);
    assert_eq!(1002, solve_for(input::INPUT).unwrap().0);
}

#[test]
fn part2() {
    assert_eq!(2021, solve_for("D2FE28").unwrap().1);
    assert_eq!(3, solve_for("C200B40A82").unwrap().1);
    assert_eq!(54, solve_for("04005AC33890").unwrap().1);
    assert_eq!(7, solve_for("880086C3E88112").unwrap().1);
    assert_eq!(9, solve_for("CE00C43D881120").unwrap().1);
    assert_eq!(1, solve_for("D8005AC2A8F0").unwrap().1);
    assert_eq!(0, solve_for("F600BC2D8F").unwrap().1);
    assert_eq!(0, solve_for("9C005AC2F8F0").unwrap().1);
    assert_eq!(1, solve_for("9C0141080250320F1802104A08").unwrap().1);
    assert_eq!(1673210814091, solve_for(input::INPUT).unwrap().1);
}
