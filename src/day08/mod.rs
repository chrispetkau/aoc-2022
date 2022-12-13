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

    /// Compute visibility of elements starting at 'start' and proceeding from there with 'inner' iterations
    /// within an 'outer' loop. Write results into 'vis_map'.
    fn compute_vis(
        &self,
        start: usize,
        outer_stride: impl Fn(usize) -> usize,
        outer_end: usize,
        inner_stride: impl Fn(usize) -> usize,
        inner_iteration_count: usize,
        vis_map: &mut [bool],
    ) {
        let mut i = start;
        while i != outer_end {
            let mut highest = self.data[i];
            vis_map[i] = true;
            let mut j = inner_stride(i);
            for _ in 0..inner_iteration_count {
                let height = self.data[j];
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
    }
}

fn part1(height_map: &HeightMap) -> usize {
    let mut vis_map = vec![false; height_map.data.len()];

    // Columns, top to bottom.
    height_map.compute_vis(
        height_map.index(0, 0),
        |index| index + 1,
        height_map.width - 1,
        |index| index + height_map.width,
        height_map.height - 2,
        &mut vis_map,
    );

    // Columns, bottom to top.
    height_map.compute_vis(
        height_map.index(0, height_map.height - 1),
        |index| index + 1,
        height_map.index(height_map.width - 1, height_map.height - 1),
        |index| index - height_map.width,
        height_map.height - 2,
        &mut vis_map,
    );

    // Rows, left to right.
    height_map.compute_vis(
        height_map.index(0, 0),
        |index| index + height_map.width,
        height_map.index(0, height_map.height - 1),
        |index| index + 1,
        height_map.width - 2,
        &mut vis_map,
    );

    // Rows, right to left.
    height_map.compute_vis(
        height_map.index(height_map.width - 1, 0),
        |index| index + height_map.width,
        height_map.index(height_map.width - 1, height_map.height - 1),
        |index| index - 1,
        height_map.width - 2,
        &mut vis_map,
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

    vis_map.iter().filter(|&&vis| vis).count()
}

fn scenic_score_in_range(
    height: u8,
    range: impl Iterator<Item = usize>,
    range_value: impl Fn(usize) -> u8,
) -> usize {
    let mut blocked = false;
    range
        .take_while(|&a| {
            let candidate_height = range_value(a);
            if candidate_height >= height {
                blocked = true;
            }
            !blocked && height > candidate_height
        })
        .count()
        + usize::from(blocked)
}

fn part2(height_map: &HeightMap) -> usize {
    let scenic_score = |x, y| -> usize {
        let height = height_map.get(x, y);
        let east = if x == height_map.width - 1 {
            0
        } else {
            scenic_score_in_range(height, x + 1..height_map.width, |x| height_map.get(x, y))
        };
        let west = if x == 0 {
            0
        } else {
            scenic_score_in_range(height, (0..x).rev(), |x| height_map.get(x, y))
        };
        let south = if y == height_map.height - 1 {
            0
        } else {
            scenic_score_in_range(height, y + 1..height_map.height, |y| height_map.get(x, y))
        };
        let north = if y == 0 {
            0
        } else {
            scenic_score_in_range(height, (0..y).rev(), |y| height_map.get(x, y))
        };
        east * west * north * south
    };
    let mut scenic_map = vec![0; height_map.data.len()];
    for x in 0..height_map.width {
        for y in 0..height_map.height {
            scenic_map[height_map.index(x, y)] = scenic_score(x, y);
        }
    }

    // Visualize.
    // for y in 0..height_map.height {
    //     for x in 0..height_map.width {
    //         print!("{} ", scenic_map[height_map.index(x, y)]);
    //     }
    //     println!();
    // }

    *scenic_map.iter().max().unwrap()
}

fn solve_for(input: &str) -> (usize, usize, Duration) {
    let timer = Instant::now();
    let height_map = input.parse::<HeightMap>().unwrap();
    let parse_duration = timer.elapsed();
    (part1(&height_map), part2(&height_map), parse_duration)
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
