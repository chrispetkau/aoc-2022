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
    data: Vec<u8>,
    start: usize,
    end: usize,
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().chars().count();
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
            data,
            start,
            end,
        })
    }
}

impl HeightMap {
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

    fn shortest_path_length(&self, start: usize) -> Result<usize> {
        let mut visited = vec![false; self.data.len()];
        visited[start] = true;
        let mut paths = vec![Path {
            index: start,
            length: 0,
        }];
        while !paths.is_empty() {
            let path = paths[0];

            paths.remove(0); // TODO bad data structure
                             // visited[path.index] = true;

            let from = self.xy_from_index(path.index);
            let length = path.length + 1;

            let mut add_path = |index: usize| {
                if !visited[index] && self.can_traverse_to(path.index, index) {
                    if index == self.end {
                        return Err(());
                    }
                    visited[index] = true;
                    paths.push(Path { index, length });
                }
                Ok(())
            };

            // north
            if let Some(index) = path.index.checked_sub(self.width) {
                if add_path(index).is_err() {
                    return Ok(length);
                }
            }

            // west
            if let Some(index) = path.index.checked_sub(1) {
                let to = self.xy_from_index(index);
                // if the y is same
                if from.1 == to.1 && add_path(index).is_err() {
                    return Ok(length);
                }
            }

            // south
            let index = path.index + self.width;
            if index < self.data.len() && add_path(index).is_err() {
                return Ok(length);
            }

            // east
            let index = path.index + 1;
            if index < self.data.len() {
                let to = self.xy_from_index(index);
                // if the y is same
                if from.1 == to.1 && add_path(index).is_err() {
                    return Ok(length);
                }
            }
        }
        Err(anyhow!("No path to end"))
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

    let part1 = height_map.shortest_path_length(height_map.start)?;

    let starts = height_map
        .data
        .iter()
        .enumerate()
        .filter_map(|(index, height)| if *height == b'a' { Some(index) } else { None })
        .collect::<Vec<_>>();
    let part2 = starts
        .iter()
        .filter_map(|start| height_map.shortest_path_length(*start).ok())
        .min()
        .unwrap();

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
