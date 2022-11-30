use self::{input::INPUT, line::Line};
use anyhow::Result;
use std::time::{Duration, Instant};

mod input;
mod line;
mod point;
mod span;

#[allow(dead_code)]
mod brute_force_solution;
#[allow(dead_code)]
mod span_solution;

#[cfg(test)]
mod tests;

fn parse(input: &str) -> Result<Vec<Line>> {
    input
        .lines()
        .map(|text_line| text_line.parse::<Line>())
        .collect::<Result<Vec<Line>>>()
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let parse_start = Instant::now();
    let lines = parse(INPUT);
    let parse_duration = Instant::now() - parse_start;
    let (part1, part2) = match lines {
        Ok(lines) => {
            brute_force_solution::solve_for(&lines)
            //span_solution::solve_for(lines)
        }
        Err(error) => {
            println!("day 5 error: {}", error);
            (0, 0)
        }
    };
    (part1, part2, parse_duration)
}
