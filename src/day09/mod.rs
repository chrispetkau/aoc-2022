use self::input::INPUT;
use std::{
    collections::HashSet,
    num::ParseIntError,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self) -> usize {
        self.x.unsigned_abs() as usize + self.y.unsigned_abs() as usize
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Motion {
    direction: Direction,
    distance: usize,
}

impl FromStr for Motion {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let direction = match tokens.next().unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!(),
        };
        let distance = tokens.next().unwrap().parse::<usize>()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

fn move_knot(head: &Point, knot: &mut Point) -> bool {
    let delta = *head - *knot;
    let manhattan_distance = delta.manhattan_distance();
    let close_enough =
        manhattan_distance < 2 || (manhattan_distance == 2 && delta.x != 0 && delta.y != 0);
    if !close_enough {
        *knot += Point {
            x: delta.x.clamp(-1, 1),
            y: delta.y.clamp(-1, 1),
        };
        return true;
    }
    false
}

fn solve_for(input: &str) -> (usize, usize, Duration) {
    let timer = Instant::now();
    let motions = input
        .lines()
        .map(|line| line.parse::<Motion>().unwrap())
        .collect::<Vec<_>>();
    let parse_duration = timer.elapsed();

    // Part 1.
    let mut head = Point { x: 0, y: 0 };
    let mut tail = head;
    let mut visited = HashSet::new();
    visited.insert(tail);
    motions.iter().for_each(|motion| {
        (0..motion.distance).for_each(|_| {
            head += motion.direction;
            if move_knot(&head, &mut tail) {
                visited.insert(tail);
            }
        });
    });
    let part1 = visited.len();

    // Part 2.
    let mut knots = vec![Point { x: 0, y: 0 }; 10];
    visited.clear();
    visited.insert(knots[0]);
    motions.iter().for_each(|motion| {
        (0..motion.distance).for_each(|_| {
            knots[0] += motion.direction;
            head = knots[0];
            (1..10).for_each(|i| {
                let knot = &mut knots[i];
                if move_knot(&head, knot) && i == 9 {
                    visited.insert(*knot);
                }
                head = *knot;
            });
        });
    });
    let part2 = visited.len();

    (part1, part2, parse_duration)
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
