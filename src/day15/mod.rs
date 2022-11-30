use anyhow::{anyhow, Result};
use std::time::{Duration, Instant};

use input::INPUT;

mod input;

#[cfg(test)]
mod tests;

/// Find the lowest-cost path and return its cost.
fn find_path(raw_cell_costs: &[u32], column_count: usize, row_count: usize) -> u32 {
    let stride = column_count + 2;
    let index = |x: usize, y: usize| x + 1 + (y + 1) * stride;
    let destination_index = index(column_count - 1, row_count - 1);
    let mut cell_costs = vec![0; stride * (row_count + 2)];
    (0..row_count).for_each(|y| {
        (0..column_count).for_each(|x| {
            cell_costs[index(x, y)] = raw_cell_costs[x + y * column_count];
        })
    });
    const COST_MAXIMUM: u32 = u32::MAX;
    let mut path_costs = vec![COST_MAXIMUM; stride * (row_count + 2)];
    path_costs[index(0, 0)] = 0;
    let mut open = vec![index(0, 0)];
    let up = -(stride as i32);
    let down = stride as i32;
    const LEFT: i32 = -1;
    const RIGHT: i32 = 1;
    let offsets = [up, down, LEFT, RIGHT];
    let mut destination_path_cost = COST_MAXIMUM;
    while let Some(index) = open.pop() {
        let path_cost = path_costs[index];

        // Once we encounter a path longer than one we've already found to the destination, we're done.
        if destination_path_cost < path_cost {
            break;
        }

        // Watch for arrival at destination, but continue to look for more solutions until the path cost is too high.
        if index == destination_index {
            if path_cost < destination_path_cost {
                destination_path_cost = path_cost;
            }
            continue;
        }

        // Extend the path in all directions.
        offsets
            .iter()
            // Filter out indices that subscript the buffered zone.
            .filter_map(|&offset| {
                let to_index = (index as i32 + offset) as usize;
                if to_index < stride
                    || cell_costs.len() - stride < to_index
                    || to_index % stride == 0
                    || (to_index + 1) % stride == 0
                {
                    None
                } else {
                    Some(to_index)
                }
            })
            .for_each(|to_index| {
                let new_path_cost = path_cost + cell_costs[to_index];
                if new_path_cost < path_costs[to_index] {
                    path_costs[to_index] = new_path_cost;

                    // Insert 'to_index' into 'open' in order.
                    let insertion_point = match open
                        .binary_search_by(|&candidate| new_path_cost.cmp(&path_costs[candidate]))
                    {
                        Ok(insertion_point) => insertion_point,
                        Err(insertion_point) => insertion_point,
                    };
                    open.insert(insertion_point, to_index);
                }
            });
    }
    destination_path_cost
}

fn solve_for(input: &str) -> Result<(u32, u32, Duration)> {
    let parse_start = Instant::now();
    let row_count = input.lines().count();
    let column_count = input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Input is empty"))?
        .chars()
        .count();
    let cell_costs = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| {
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("Failed to parse digit"))
            })
        })
        .collect::<Result<Vec<u32>>>()?;
    let parse_duration = parse_start.elapsed();

    let index = |x, y| x + y * column_count;

    let part1 = find_path(&cell_costs, column_count, row_count);

    // Build 5x5 tiled map of cell costs and submit the result to find_path() for part2.
    const TILES_WIDTH: usize = 5;
    const TILES_HEIGHT: usize = 5;
    let cells_cost_x5_stride = TILES_WIDTH * column_count;
    let mut cell_costs_x5 = vec![0; row_count * TILES_HEIGHT * cells_cost_x5_stride];
    let index_in_tile = |(tile_x, tile_y), (x, y)| {
        (x + tile_x * column_count) + (y + tile_y * row_count) * cells_cost_x5_stride
    };
    let cell_value = |(tile_x, tile_y), (x, y)| {
        let cell_cost = cell_costs[index(x, y)];
        let tiled_cell_cost = cell_cost + ((tile_x + tile_y) as u32);
        ((tiled_cell_cost - 1) % 9) + 1
    };
    (0..TILES_HEIGHT).for_each(|tile_y| {
        (0..TILES_WIDTH).for_each(|tile_x| {
            (0..row_count).for_each(|y| {
                (0..column_count).for_each(|x| {
                    let index = index_in_tile((tile_x, tile_y), (x, y));
                    let cell_value = cell_value((tile_x, tile_y), (x, y));
                    cell_costs_x5[index] = cell_value;
                });
            })
        })
    });
    let part2 = find_path(&cell_costs_x5, column_count * 5, row_count * 5);

    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (u32, u32, Duration) {
    match solve_for(INPUT) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 15 error: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}
