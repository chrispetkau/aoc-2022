use anyhow::{anyhow, Result};
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
    start: usize,
    end: usize,
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();
        let mut data = s.lines().flat_map(|line| line.bytes()).collect::<Vec<_>>();
        let start = data
            .iter()
            .enumerate()
            .find_map(|(index, c)| if *c == b'S' { Some(index) } else { None })
            .unwrap();
        data[start] = b'a';
        let end = data
            .iter()
            .enumerate()
            .find_map(|(index, c)| if *c == b'E' { Some(index) } else { None })
            .unwrap();
        data[end] = b'z';
        Ok(Self {
            width,
            height,
            data,
            start,
            end,
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

    fn xy_from_index(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn can_traverse_to(&self, from: usize, to: usize) -> bool {
        if let Some(step) = self.data[to].checked_sub(self.data[from]) {
            step <= 1
        } else {
            true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Path {
    index: usize,
    length: usize,
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let parse_start = Instant::now();
    let height_map = input.parse::<HeightMap>()?;
    let parse_duration = Instant::now() - parse_start;

    let mut visited = vec![false; height_map.data.len()];
    visited[height_map.start] = true;
    let mut paths = vec![Path {
        index: height_map.start,
        length: 0,
    }];
    let mut shortest_path_length = None;
    while !paths.is_empty() {
        let path = paths[0];

        paths.remove(0); // TODO bad data structure
        // visited[path.index] = true;

        let from = height_map.xy_from_index(path.index);
        let length = path.length + 1;

        let mut add_path = |index: usize| {
            if !visited[index] && height_map.can_traverse_to(path.index, index) {
                if index == height_map.end {
                    return Err(());
                }
                visited[index] = true;
                paths.push(Path { index, length });
            }
            Ok(())
        };

        // north
        if let Some(index) = path.index.checked_sub(height_map.width) {
            if add_path(index).is_err() {
                shortest_path_length = Some(length);
                break;
            }
        }

        // west
        if let Some(index) = path.index.checked_sub(1) {
            let to = height_map.xy_from_index(index);
            // if the y is same
            if from.1 == to.1 && add_path(index).is_err() {
                shortest_path_length = Some(length);
                break;
            }
        }

        // south
        let index = path.index + height_map.width;
        if index < height_map.data.len() && add_path(index).is_err() {
            shortest_path_length = Some(length);
            break;
        }

        // east
        let index = path.index + 1;
        if index < height_map.data.len() {
            let to = height_map.xy_from_index(index);
            // if the y is same
            if from.1 == to.1 && add_path(index).is_err() {
                shortest_path_length = Some(length);
                break;
            }
        }
    }

    let part1 = shortest_path_length.unwrap();
    let part2 = 0;
    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    match solve_for(INPUT) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 12 error: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}
