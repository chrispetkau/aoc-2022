use self::input::INPUT;
use anyhow::Result;
use std::time::{Duration, Instant};

mod input;

#[cfg(test)]
mod tests;

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

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let inventory_lists = parse(input)?;
    let parse_duration = timer.elapsed();
    let mut inventory_sums = inventory_lists
        .iter()
        .map(|inventory| inventory.iter().sum::<usize>())
        .collect::<Vec<_>>();
    inventory_sums.sort();
    let largest_inventory = inventory_sums.iter().rev().take(1).sum::<usize>();
    let largest_three = inventory_sums.iter().rev().take(3).sum::<usize>();
    Ok((largest_inventory, largest_three, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let (part1, part2, parse_duration) = solve_for(INPUT).unwrap();
    (part1, part2, parse_duration)
}
