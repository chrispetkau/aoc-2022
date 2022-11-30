use anyhow::{anyhow, Result};
use input::INPUT;
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum CaveSize {
    Big,
    Small,
}

// TODO eugh...strings as ids...change this to a serial number and make this type Copy so we don't need to
// explicitly clone() everywhere
#[derive(Clone, PartialEq, Eq, Debug)]
struct Cave(CaveSize, String);

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_lowercase()) {
            Ok(Self {
                0: CaveSize::Small,
                1: String::from(s),
            })
        } else if s.chars().all(|c| c.is_ascii_uppercase()) {
            Ok(Self {
                0: CaveSize::Big,
                1: String::from(s),
            })
        } else {
            Err(anyhow!("Can't parse into CaveId: [{}]", s))
        }
    }
}

#[derive(Clone, Debug)]
struct Tunnel(Cave, Cave);

impl FromStr for Tunnel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut caves = s.split('-');
        Ok(Self {
            0: caves
                .next()
                .ok_or_else(|| anyhow!("Missing 'from' cave"))?
                .parse::<Cave>()?,
            1: caves
                .next()
                .ok_or_else(|| anyhow!("Missing 'to' cave"))?
                .parse::<Cave>()?,
        })
    }
}

fn count_paths(tunnels: &[Tunnel], permit_double_small_cave_traversal_once: bool) -> usize {
    let start_cave = String::from("start");
    let end_cave = String::from("end");

    let mut pending_paths = tunnels
        .iter()
        .filter_map(|tunnel| {
            if tunnel.0 .1 == start_cave {
                Some((
                    vec![tunnel.0.clone(), tunnel.1.clone()],
                    !permit_double_small_cave_traversal_once,
                ))
            } else if tunnel.1 .1 == start_cave {
                Some((
                    vec![tunnel.1.clone(), tunnel.0.clone()],
                    !permit_double_small_cave_traversal_once,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<(Vec<Cave>, bool)>>();
    let mut paths = Vec::new();
    while let Some(path) = pending_paths.pop() {
        let from = path.0.last().unwrap();
        if from.1 == end_cave {
            paths.push(path);
            continue;
        }
        tunnels
            .iter()
            .filter_map(|tunnel| {
                if tunnel.0 == *from {
                    Some(&tunnel.1)
                } else if tunnel.1 == *from {
                    Some(&tunnel.0)
                } else {
                    None
                }
            })
            .filter_map(|cave| match cave {
                Cave(CaveSize::Small, id) => {
                    let traversed = path.0.contains(cave);
                    if *id == start_cave || *id == end_cave || path.1 {
                        if !traversed {
                            Some((cave, false))
                        } else {
                            None
                        }
                    } else {
                        Some((cave, traversed))
                    }
                }
                _ => Some((cave, false)),
            })
            .for_each(|(to, use_double_small_cave_traversal)| {
                let mut new_path = path.clone();
                new_path.0.push(to.clone());
                if use_double_small_cave_traversal {
                    new_path.1 = true;
                }
                pending_paths.push(new_path);
            });
    }
    paths.len()
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let parse_start = Instant::now();
    let tunnels = input
        .lines()
        .map(|line| line.parse::<Tunnel>())
        .collect::<Result<Vec<Tunnel>>>()?;
    let parse_duration = Instant::now() - parse_start;

    let part1 = count_paths(&tunnels, false);
    let part2 = count_paths(&tunnels, true);
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
