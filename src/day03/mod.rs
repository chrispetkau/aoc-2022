use self::input::INPUT;
use std::{collections::HashSet, time::Duration};

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
    let mut items = HashSet::new();
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            items.clear();
            a.chars().for_each(|x| {
                items.insert(x);
            });
            let duplicate = b.chars().find(|x| items.contains(x)).unwrap();
            priority(duplicate) as usize
        })
        .sum::<usize>()
}

fn solve_part2(input: &str) -> usize {
    let line_count = input.lines().count();
    let mut lines = input.lines();
    let mut i = 0;
    let mut sum = 0;

    let mut a_items = HashSet::new();
    let mut b_items = HashSet::new();
    while i != line_count {
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        a_items.clear();
        a.chars().for_each(|x| {
            a_items.insert(x);
        });
        b_items.clear();
        b.chars().for_each(|x| {
            b_items.insert(x);
        });
        let duplicate = c
            .chars()
            .find(|x| a_items.contains(x) && b_items.contains(x))
            .unwrap();
        sum += priority(duplicate) as usize;

        i += 3;
    }
    sum
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    (solve_part1(INPUT), solve_part2(INPUT), Duration::new(0, 0))
}
