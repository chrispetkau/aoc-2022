use input::INPUT;
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            width: s.lines().next().unwrap().chars().count(),
            height: s.lines().count(),
            data: s
                .lines()
                .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
                .collect::<Vec<_>>(),
        })
    }
}

impl HeightMap {
    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[self.index(x, y)]
    }
}

fn solve_for(input: &str) -> (usize, usize, Duration) {
    let timer = Instant::now();
    let height_map = input.parse::<HeightMap>().unwrap();
    let parse_duration = timer.elapsed();
    let mut vis_map = vec![false; height_map.data.len()];

    let mut compute_vis = |start,
                           outer_stride: &dyn Fn(usize) -> usize,
                           outer_end,
                           inner_stride: &dyn Fn(usize) -> usize,
                           inner_iteration_count| {
        let mut i = start;
        while i != outer_end {
            let mut highest = height_map.data[i];
            vis_map[i] = true;
            let mut j = inner_stride(i);
            for _ in 0..inner_iteration_count {
                let height = height_map.data[j];
                if height > highest {
                    vis_map[j] = true;
                    highest = height;
                }
                if highest == 9 {
                    break;
                }
                j = inner_stride(j);
            }
            i = outer_stride(i);
        }
    };

    // Columns, top to bottom.
    compute_vis(
        height_map.index(0, 0),
        &|index| index + 1,
        height_map.width - 1,
        &|index| index + height_map.width,
        height_map.height - 2,
    );

    // Columns, bottom to top.
    compute_vis(
        height_map.index(0, height_map.height - 1),
        &|index| index + 1,
        height_map.index(height_map.width - 1, height_map.height - 1),
        &|index| index - height_map.width,
        height_map.height - 2,
    );

    // Rows, left to right.
    compute_vis(
        height_map.index(0, 0),
        &|index| index + height_map.width,
        height_map.index(0, height_map.height - 1),
        &|index| index + 1,
        height_map.width - 2,
    );

    // Rows, right to left.
    compute_vis(
        height_map.index(height_map.width - 1, 0),
        &|index| index + height_map.width,
        height_map.index(height_map.width - 1, height_map.height - 1),
        &|index| index - 1,
        height_map.width - 2,
    );

    // Corners.
    vis_map[height_map.index(0, 0)] = true;
    vis_map[height_map.index(0, height_map.height - 1)] = true;
    vis_map[height_map.index(height_map.width - 1, 0)] = true;
    vis_map[height_map.index(height_map.width - 1, height_map.height - 1)] = true;

    // Visualize.
    // for y in 0..height_map.height {
    //     for x in 0..height_map.width {
    //         print!(
    //             "{} ",
    //             if vis_map[height_map.index(x, y)] {
    //                 '.'
    //             } else {
    //                 ' '
    //             }
    //         );
    //     }
    //     println!();
    // }

    let part1 = vis_map.iter().filter(|&&vis| vis).count();

    (part1, 0, parse_duration)
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
