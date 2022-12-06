use self::input::INPUT;
use anyhow::Result;
use std::{
    num::ParseIntError,
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

/// Inclusive range.
struct Range {
    from: u8,
    to: u8,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut section_range = s.split('-').map(|s| s.parse::<u8>());
        Ok(Self {
            from: section_range.next().unwrap()?,
            to: section_range.next().unwrap()?,
        })
    }
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        !(self.from > other.to || self.to < other.from)
    }
}

fn symmetric_contains(a: &Range, b: &Range) -> bool {
    if a.from <= b.from {
        if a.to >= b.to {
            true // a contains b
        } else {
            a.from == b.from // b contains a
        }
    } else {
        a.to <= b.to
    }
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let elves = input
        .lines()
        .map(|line| {
            let mut elves = line.split(',').map(|s| s.parse::<Range>());
            (
                elves.next().unwrap().unwrap(),
                elves.next().unwrap().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let parse_duration = timer.elapsed();
    let part1 = elves
        .iter()
        .filter(|(a, b)| symmetric_contains(a, b))
        .count();
    let part2 = elves.iter().filter(|(a, b)| a.overlaps(b)).count();
    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    match solve_for(INPUT) {
        Ok(answer) => answer,
        Err(error) => {
            println!("Error solving day 4: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}