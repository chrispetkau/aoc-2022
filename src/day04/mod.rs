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
        if a.to <= b.to {
            true // b contains a
        } else {
            false
        }
    }
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let parse_duration = timer.elapsed();
    let part1 = input
        .lines()
        .filter(|line| {
            let mut elves = line.split(',').map(|s| s.parse::<Range>());
            symmetric_contains(
                &elves.next().unwrap().unwrap(),
                &elves.next().unwrap().unwrap(),
            )
        })
        .count();
    let part2 = input
        .lines()
        .filter(|line| {
            let mut elves = line.split(',').map(|s| s.parse::<Range>());
            elves
                .next()
                .unwrap()
                .unwrap()
                .overlaps(&elves.next().unwrap().unwrap())
        })
        .count();
    Ok((part1, part2, parse_duration))
}

// TODO figure out why this is so much slower than Chris Ozeroff's solution
pub(crate) fn solve() -> (usize, usize, Duration) {
    match solve_for(INPUT) {
        Ok(answer) => answer,
        Err(error) => {
            println!("Error solving day 4: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}
