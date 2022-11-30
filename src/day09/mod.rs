use self::input::INPUT;
use std::{
    cmp::Ordering,
    collections::HashSet,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

const LEFT: u8 = 1 << 0;
const RIGHT: u8 = 1 << 1;
const UP: u8 = 1 << 2;
const DOWN: u8 = 1 << 3;
const ALL: u8 = LEFT | RIGHT | UP | DOWN;
const TERMINAL: u8 = 1 << 4;

fn set_left_right(transitions: &mut [u8], left: usize, right: usize, left_cmp_right: Ordering) {
    match left_cmp_right {
        Ordering::Less => transitions[right] |= LEFT,
        Ordering::Equal => {
            transitions[right] |= LEFT;
            transitions[left] |= RIGHT;
        }
        Ordering::Greater => transitions[left] |= RIGHT,
    }
}

fn set_up_down(transitions: &mut [u8], up: usize, down: usize, up_cmp_down: Ordering) {
    match up_cmp_down {
        Ordering::Less => transitions[down] |= UP,
        Ordering::Equal => {
            transitions[down] |= UP;
            transitions[up] |= DOWN;
        }
        Ordering::Greater => transitions[up] |= DOWN,
    }
}

struct FloodFiller<'a> {
    transitions: &'a [u8],
    stride: usize,
    closed: HashSet<usize>,
    open: Vec<usize>,
}

impl FloodFiller<'_> {
    fn execute(&mut self, start: usize) -> usize {
        self.closed.clear();
        self.open.clear();
        self.open.push(start);
        let mut count = 0;
        while let Some(index) = self.open.pop() {
            if !self.closed.insert(index) {
                continue;
            }

            let transition_mask = self.transitions[index];
            if (transition_mask & TERMINAL) != 0 {
                continue;
            }

            count += 1;

            // Invert the transition mask to determine ascending directions in which
            // to travel *from* the minimum points.
            let transition_mask = transition_mask ^ ALL;

            if (transition_mask & LEFT) != 0 {
                self.open.push(index - 1);
            }
            if (transition_mask & RIGHT) != 0 {
                self.open.push(index + 1);
            }
            if (transition_mask & UP) != 0 {
                self.open.push(index - self.stride);
            }
            if (transition_mask & DOWN) != 0 {
                self.open.push(index + self.stride);
            }
        }
        count
    }
}

fn solve_for(input: &str) -> (usize, usize, Duration) {
    let row_count = input.lines().count();
    let column_count = input.lines().next().unwrap().len();
    let stride = column_count + 2;
    let index = |x: usize, y: usize| x + 1 + ((y + 1) * stride);

    // Buffer the height map with extra rows and columns on the edges of the maximum height.
    const HEIGHT_MAX: u8 = 9;
    let mut heights = vec![HEIGHT_MAX; (row_count + 2) * stride];

    // Read in the height map data.
    let parse_start = Instant::now();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, height)| heights[index(x, y)] = height.to_digit(10).unwrap() as u8);
    });
    let parse_duration = Instant::now() - parse_start;

    // Build a buffered array of transitions.
    let mut transitions: Vec<u8> = vec![0b11111; heights.len()];
    (1..column_count + 1)
        .for_each(|x| (1..row_count + 1).for_each(|y| transitions[x + y * stride] = 0));

    // Iterate over all cells except the buffer rows and columns, checking neighbors in all directions.
    // A direction bit is set if height decreases, or is equal, in that direction.
    // Thus minimum points will have no bits set.
    (1..column_count + 1).for_each(|x| {
        (1..row_count + 1).for_each(|y| {
            let current = x + y * stride;
            let height = heights[current];

            let right = current + 1;
            set_left_right(
                &mut transitions,
                current,
                right,
                height.cmp(&heights[right]),
            );

            let left = current - 1;
            set_left_right(&mut transitions, left, current, heights[left].cmp(&height));

            let down = current + stride;
            set_up_down(&mut transitions, current, down, height.cmp(&heights[down]));

            let up = current - stride;
            set_up_down(&mut transitions, up, current, heights[up].cmp(&height));

            if height == HEIGHT_MAX {
                transitions[current] |= TERMINAL;
            }
        });
    });

    let indices = (0..column_count).flat_map(|x| (0..row_count).map(move |y| index(x, y)));

    let minima = indices
        .filter_map(|index| {
            if transitions[index] == 0 {
                Some((index, (heights[index] + 1) as usize))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let part1 = minima
        .iter()
        .fold(0, |current, (_index, risk_level)| current + risk_level);

    let mut flood_filler = FloodFiller {
        transitions: &transitions,
        open: Vec::new(),
        closed: HashSet::new(),
        stride,
    };
    let mut basin_sizes = minima
        .iter()
        .map(|(index, _risk_level)| flood_filler.execute(*index))
        .collect::<Vec<usize>>();
    basin_sizes.sort_unstable();
    let part2 = basin_sizes.iter().rev().take(3).product();

    (part1, part2, parse_duration)
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
