use anyhow::{anyhow, Result};
use input::INPUT;
use std::{
    fmt::Display,
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = s.split(',');
        Ok(Self {
            x: digits
                .next()
                .ok_or_else(|| anyhow!("No first digit"))?
                .parse::<usize>()
                .map_err(|error| anyhow!("Error parsing first digit: {}", error))?,
            y: digits
                .next()
                .ok_or_else(|| anyhow!("No second digit"))?
                .parse::<usize>()
                .map_err(|error| anyhow!("Error parsing second digit: {}", error))?,
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Copy, Clone, Debug)]
struct Fold {
    axis: Axis,
    value: usize,
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut axis = s
            .split(' ')
            .last()
            .ok_or_else(|| anyhow!("Input string is empty?"))?
            .split('=');
        Ok(Self {
            axis: match axis.next().ok_or_else(|| anyhow!("Can't find fold axis"))? {
                "x" => Ok(Axis::X),
                "y" => Ok(Axis::Y),
                axis => Err(anyhow!("Unrecognized axis: {}", axis)),
            }?,
            value: axis
                .next()
                .ok_or_else(|| anyhow!("No axis value"))?
                .parse::<usize>()
                .map_err(|error| anyhow!("Failed to parse fold axis value: {}", error))?,
        })
    }
}

#[derive(Debug)]
struct Paper {
    size: Coord,
    cells: Vec<bool>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.size.y).try_for_each(|y| {
            (0..self.size.x).try_for_each(|x| {
                write!(
                    f,
                    "{}",
                    if self.cells[x + y * self.size.x] {
                        '#'
                    } else {
                        '.'
                    }
                )
            })?;
            writeln!(f)
        })
    }
}

fn fold_up(paper: Paper, axis: usize) -> Paper {
    let paper_start = axis + 1;
    let fold_last = axis - 1;
    let folded_size = Coord::new(paper.size.x, axis.max(paper.size.y - paper_start));
    let mut folded_paper = Paper {
        size: folded_size,
        cells: paper.cells[0..folded_size.x * folded_size.y].to_vec(),
    };
    (paper_start..paper.size.y)
        .enumerate()
        .filter_map(|(index, y)| {
            if index <= fold_last {
                Some((fold_last - index, y))
            } else {
                None
            }
        })
        .for_each(|(fold_y, y)| {
            let fold_start = fold_y * folded_size.x;
            let paper_row_start = y * paper.size.x;
            (0..paper.size.x).for_each(|x| {
                let cell = &mut folded_paper.cells[fold_start + x];
                if !*cell {
                    *cell = paper.cells[paper_row_start + x];
                }
            });
        });
    folded_paper
}

fn fold_left(paper: Paper, axis: usize) -> Paper {
    let paper_start = axis + 1;
    let fold_last = axis - 1;
    let folded_size = Coord::new(axis.max(paper.size.x - paper_start), paper.size.y);
    let mut folded_paper = Paper {
        size: folded_size,
        cells: (0..paper.size.y)
            .flat_map(|y| (0..axis).map(move |x| x + y * paper.size.x))
            .map(|index| paper.cells[index])
            .collect::<Vec<bool>>(),
    };
    (paper_start..paper.size.x)
        .enumerate()
        .filter_map(|(index, x)| {
            if index <= fold_last {
                Some((fold_last - index, x))
            } else {
                None
            }
        })
        .for_each(|(fold_x, x)| {
            (0..paper.size.y).for_each(|y| {
                let cell = &mut folded_paper.cells[fold_x + y * folded_size.x];
                if !*cell {
                    *cell = paper.cells[x + y * paper.size.x];
                }
            });
        });
    folded_paper
}

fn solve_for(input: &str) -> Result<(usize, String, Duration)> {
    let parse_start = Instant::now();
    let mut sections = input.split("\n\n");
    let dots = sections
        .next()
        .ok_or_else(|| anyhow!("No dots section"))?
        .lines()
        .map(|line| line.parse::<Coord>())
        .collect::<Result<Vec<Coord>>>()?;
    let folds = sections
        .next()
        .ok_or_else(|| anyhow!("No folds section"))?
        .lines()
        .map(|line| line.parse::<Fold>())
        .collect::<Result<Vec<Fold>>>()?;
    let parse_duration = parse_start.elapsed();

    let size = dots.iter().fold(Coord::new(0, 0), |current, coord| Coord {
        x: current.x.max(coord.x + 1),
        y: current.y.max(coord.y + 1),
    });
    let mut paper = Paper {
        size,
        cells: vec![false; size.x * size.y],
    };
    dots.iter()
        .for_each(|coord| paper.cells[coord.x + coord.y * size.x] = true);

    let first_fold = folds
        .get(0)
        .ok_or_else(|| anyhow!("No folds whatsoever!"))?;
    let paper = match first_fold.axis {
        Axis::X => fold_left(paper, first_fold.value),
        Axis::Y => fold_up(paper, first_fold.value),
    };
    let part1 = paper.cells.iter().filter(|&&cell| cell).count();

    let part2 = format!(
        "{}",
        folds
            .iter()
            .skip(1)
            .fold(paper, |current, fold| match fold.axis {
                Axis::X => fold_left(current, fold.value),
                Axis::Y => fold_up(current, fold.value),
            })
    );

    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, String, Duration) {
    match solve_for(INPUT) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 13 error: {}", error);
            (0, String::new(), Duration::new(0, 0))
        }
    }
}
