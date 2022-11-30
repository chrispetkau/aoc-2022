use super::{decode, input, solve_for, DIGIT_COUNT, WIRE_COUNT};

const INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[test]
fn part1() {
    assert_eq!(26, solve_for(INPUT).0);
    assert_eq!(493, solve_for(input::INPUT).0);
}

#[test]
fn get_candidates() {
    const ALL: usize = 0b1111111111;
    (0..DIGIT_COUNT).for_each(|digit| {
        assert_eq!(
            ALL,
            (1..=WIRE_COUNT).fold(0b0, |current, matching_segment_count| {
                let candidate_mask = super::get_candidates(digit, matching_segment_count);
                if candidate_mask != ALL {
                    assert!((current & candidate_mask) == 0,
                            "No candidates should be present yet, digit [{}], matching_segment_count [{}]",
                            digit,
                            matching_segment_count);
                    current | candidate_mask
                } else {
                    current
                }
            }),
            "Not all bits set for digit [{}]",
            digit
        );
    });
}

#[test]
fn part2_mechanics() {
    let signals_digits = [
        ("acedgfb", 8),
        ("cdfbe", 5),
        ("gcdfa", 2),
        ("fbcad", 3),
        ("dab", 7),
        ("cefabd", 9),
        ("cdfgeb", 6),
        ("eafb", 4),
        ("cagedb", 0),
        ("ab", 1),
    ];
    let outputs = ["cdfeb", "fcadb", "cdfeb", "cdbaf"];
    let (signals, _digits): (Vec<&str>, Vec<usize>) = signals_digits.iter().copied().unzip();
    assert_eq!(5353, decode(&signals, &outputs));
}

#[test]
fn part2() {
    assert_eq!(61229, solve_for(INPUT).1);
    assert_eq!(1010460, solve_for(input::INPUT).1);
}
