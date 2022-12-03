use self::input::INPUT;
use std::{cmp::Ordering, ops::Range, time::Duration};

mod input;

#[cfg(test)]
mod tests;

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => panic!(),
    }
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let duplicate = a
                .chars()
                .find_map(|c| b.chars().find(|&b_c| c == b_c))
                .unwrap();
            let priority_of_duplicate = priority(duplicate);
            priority_of_duplicate as usize
        })
        .sum::<usize>()
}

fn solve_part2(input: &str) -> usize {
    0
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    (solve_part1(INPUT), solve_part2(INPUT), Duration::new(0, 0))
}
