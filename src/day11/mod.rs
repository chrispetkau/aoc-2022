use self::input::INPUT;
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

fn solve_for(input: &str) -> (usize, usize, Duration) {
    const SIZE: usize = 10;
    const STRIDE: usize = SIZE + 2;
    const BUFFERED_SIZE: usize = STRIDE.pow(2);
    let mut buffered_energy_levels: [u8; BUFFERED_SIZE] = [0; BUFFERED_SIZE];

    // Index into buffered_energy_levels for the specified energy_levels coordinate.
    let index = |x: usize, y: usize| x + 1 + (y + 1) * STRIDE;
    let indices = (0..SIZE).flat_map(|y| (0..SIZE).map(move |x| index(x, y)));

    // Parse input data into buffered_energy_levels.
    let parse_start = Instant::now();
    let input = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect::<Vec<u8>>();
    let parse_duration = Instant::now() - parse_start;

    indices
        .clone()
        .zip(input.iter())
        .for_each(|(i, &initial_energy_level)| buffered_energy_levels[i] = initial_energy_level);

    const STRIDE_AS_I32: i32 = STRIDE as i32;
    const NEIGHBOR_OFFSETS: [i32; 8] = [
        -STRIDE_AS_I32 - 1,
        -STRIDE_AS_I32,
        1 - STRIDE_AS_I32,
        -1,
        1,
        STRIDE_AS_I32 - 1,
        STRIDE_AS_I32,
        STRIDE_AS_I32 + 1,
    ];
    let mut flashed = HashSet::new();
    let mut sim = || {
        indices.clone().for_each(|i| buffered_energy_levels[i] += 1);
        flashed.clear();
        let flash_count = loop {
            let flashers = indices
                .clone()
                .filter(|&i| 9 < buffered_energy_levels[i])
                .filter(|&i| flashed.insert(i))
                .collect::<Vec<usize>>();
            if flashers.is_empty() {
                break flashed.len();
            }
            flashers.iter().for_each(|&i| {
                NEIGHBOR_OFFSETS.iter().for_each(|&offset| {
                    let neighbor_index = (i as i32 + offset) as usize;
                    buffered_energy_levels[neighbor_index] += 1
                });
            });
        };
        flashed.iter().for_each(|&i| buffered_energy_levels[i] = 0);
        flash_count
    };

    let mut part1 = 0;
    let mut part2 = None;
    let mut sim_count = 0;
    const PART1_SIM_COUNT: usize = 100;
    const OCTOPUS_COUNT: usize = SIZE.pow(2);
    loop {
        let done_part1 = PART1_SIM_COUNT <= sim_count;
        let flash_count = sim();
        sim_count += 1;
        if !done_part1 {
            part1 += flash_count;
            if part2.is_some() {
                break;
            }
        }
        if flash_count == OCTOPUS_COUNT && part2.is_none() {
            part2 = Some(sim_count);
            if done_part1 {
                break;
            }
        }
    }

    (part1, part2.unwrap(), parse_duration)
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
