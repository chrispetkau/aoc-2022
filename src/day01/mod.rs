use self::input::INPUT;
use anyhow::{anyhow, Result};
use std::{
    num::ParseIntError,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

fn count_increasing(input: &[usize], step_size: usize) -> usize {
    input
        .iter()
        .zip(input.iter().skip(step_size))
        .filter(|(a, b)| a < b)
        .count()
}

fn parse(input: &str) -> Result<Vec<Vec<usize>>> {
    let mut inventory_lists = vec![];
    let mut inventory = vec![];
    for line in input.lines() {
        if line.is_empty() {
            inventory_lists.push(inventory);
            inventory = vec![];
        } else {
            inventory.push(line.parse::<usize>()?);
        }
    }
    inventory_lists.push(inventory);
    Ok(inventory_lists)
}

fn solve_for(input: &str) -> Result<(usize, Duration)> {
    let timer = Instant::now();
    let inventory_lists = parse(input)?;
    let parse_duration = timer.elapsed();
    let inventory_sums = inventory_lists
        .iter()
        .map(|inventory| inventory.iter().sum::<usize>());
    let largest_inventory = inventory_sums
        .max()
        .ok_or_else(|| anyhow!("no largest inventory"))?;
    Ok((largest_inventory, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let (part1, part1_parse_duration) = solve_for(INPUT).unwrap();
    (part1, 0, part1_parse_duration)
}
