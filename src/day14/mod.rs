use anyhow::{anyhow, Result};
use std::{
    num::ParseIntError,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
    time::{Duration, Instant},
};

use self::input::INPUT;

mod input;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(',');
        let x = tokens.next().unwrap().parse::<usize>()?;
        let y = tokens.next().unwrap().parse::<usize>()?;
        Ok(Self { x, y })
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x as i32 - rhs.x as i32,
            y: self.y as i32 - rhs.y as i32,
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x = (self.x as i32 + rhs.x) as usize;
        self.y = (self.y as i32 + rhs.y) as usize;
    }
}

// TODO any way to do this generically via Into?
impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self += Vector::from(rhs)
    }
}

impl Add<Vector> for Point {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: (self.x as i32 + rhs.x) as usize,
            y: (self.y as i32 + rhs.y) as usize,
        }
    }
}

// TODO any way to do this generically via Into?
impl Add<Direction> for Point {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        self + Vector::from(rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl From<Direction> for Vector {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self { x: 0, y: -1 },
            Direction::Down => Self { x: 0, y: 1 },
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_vector(v: Vector) -> Option<Self> {
        if v.x < 0 {
            Some(Self::Left)
        } else if v.x > 0 {
            Some(Self::Right)
        } else if v.y < 0 {
            Some(Self::Up)
        } else if v.y > 0 {
            Some(Self::Down)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Polyline {
    start: Point,
    segments: Vec<(Direction, usize)>,
}

impl FromStr for Polyline {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ");
        let start = points.next().unwrap().parse::<Point>()?;
        let mut prev = start;
        let segments = points
            .map(|point| -> Result<(Direction, usize), anyhow::Error> {
                let point = point.parse::<Point>()?;
                let delta = point - prev;
                prev = point;
                let direction = Direction::from_vector(delta)
                    .ok_or_else(|| anyhow!("Delta doesn't map to a Direction. Probably zero."))?;
                let distance = (delta.x.abs() + delta.y.abs()) as usize;
                Ok((direction, distance))
            })
            .collect::<Result<Vec<(Direction, usize)>, anyhow::Error>>()?;
        Ok(Self { start, segments })
    }
}

fn simulate(width: usize, height: usize, x_min: usize, grid: &mut [bool]) -> usize {
    let index = |point: &Point| (point.y * width) + (point.x - x_min);
    const SAND_ENTRY_POINT: Point = Point { x: 500, y: 0 };
    let sand_entry_index = index(&SAND_ENTRY_POINT);
    let mut count = 0;
    loop {
        if grid[sand_entry_index] {
            break;
        }
        let mut sand = SAND_ENTRY_POINT;
        let settled = loop {
            if sand.y == height - 1 {
                break false;
            }
            let below = sand + Direction::Down;
            if !grid[index(&below)] {
                sand = below;
            } else {
                if sand.x - x_min == 0 {
                    break false;
                }
                let next = below + Direction::Left;
                if !grid[index(&next)] {
                    sand = next;
                } else {
                    if sand.x - x_min == width - 1 {
                        break false;
                    }
                    let next = below + Direction::Right;
                    if !grid[index(&next)] {
                        sand = next;
                    } else {
                        break true;
                    }
                }
            }
        };
        if settled {
            grid[index(&sand)] = true;
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let polylines = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| point.parse::<Point>())
                .collect::<Result<Vec<_>, ParseIntError>>()
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    let parse_duration = timer.elapsed();

    let (x_min, x_max, y_max) = polylines.iter().flatten().fold(
        (500, 500, 0),
        |(mut x_min, mut x_max, mut y_max), point| {
            if point.x < x_min {
                x_min = point.x;
            }
            if x_max < point.x {
                x_max = point.x;
            }
            if y_max < point.y {
                y_max = point.y;
            }
            (x_min, x_max, y_max)
        },
    );
    // Part 1 dimensions.
    let width = x_max - x_min + 1;
    let height = y_max + 1;

    // Add floor for part 2, and inflate the grid accordingly.
    let floor_depth = height + 2;
    let floor_length = (floor_depth + 1) * 3; // arbitrary multiple just to ensure it is big enough
    let inflation = floor_length - width;
    let x_min = x_min - inflation / 2;
    let width = floor_length;
    let height = floor_depth;

    let mut grid = vec![false; width * height];

    let index = |point: &Point| (point.y * width) + (point.x - x_min);

    // TODO double-parsing
    let polylines = input
        .lines()
        .map(|line| line.parse::<Polyline>())
        .collect::<Result<Vec<_>>>()?;

    polylines.iter().for_each(|polyline| {
        let mut point = polyline.start;
        grid[index(&point)] = true;
        // println!("{point:?}");
        polyline.segments.iter().for_each(|(direction, distance)| {
            // println!("{direction:?} {distance}");
            let delta = Vector::from(*direction);
            (0..*distance).for_each(|_| {
                point += delta;
                // println!("{point:?}");
                grid[index(&point)] = true;
            });
        });
    });

    let part1 = simulate(width, height, x_min, &mut grid.clone());

    let floor_start = Point {
        x: x_min,
        y: floor_depth - 1,
    };
    let start_index = index(&floor_start);
    grid.iter_mut()
        .skip(start_index)
        .take(floor_length)
        .for_each(|cell| *cell = true);
    let part2 = simulate(width, height, x_min, &mut grid);

    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT).unwrap()
}
