use self::input::INPUT;
use std::time::{Duration, Instant};

mod input;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!(),
        }
    }
}

impl From<Shape> for usize {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!(),
        }
    }
}

impl From<Outcome> for usize {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

impl Outcome {
    fn manifest(self, opponent_shape: Shape) -> Shape {
        match self {
            Outcome::Win => match opponent_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Loss => match opponent_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => opponent_shape,
        }
    }
}

fn outcome(opponent_shape: Shape, my_shape: Shape) -> Outcome {
    match opponent_shape {
        Shape::Rock => match my_shape {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Loss,
        },
        Shape::Paper => match my_shape {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Win,
        },
        Shape::Scissors => match my_shape {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Loss,
            Shape::Scissors => Outcome::Draw,
        },
    }
}

fn score(my_shape: Shape, outcome: Outcome) -> usize {
    usize::from(my_shape) + usize::from(outcome)
}

fn parse_part1(input: &str) -> Vec<(Shape, Shape)> {
    input
        .lines()
        .map(|line| {
            (
                Shape::from(line.chars().next().unwrap()),
                Shape::from(line.chars().nth(2).unwrap()),
            )
        })
        .collect::<Vec<_>>()
}

fn solve_part1(input: &str) -> (usize, Duration) {
    let timer = Instant::now();
    let games = parse_part1(input);
    let parse_duration = timer.elapsed();
    (
        games
            .iter()
            .map(|&(opponent_shape, my_shape)| score(my_shape, outcome(opponent_shape, my_shape)))
            .sum::<usize>(),
        parse_duration,
    )
}

fn parse_part2(input: &str) -> Vec<(Shape, Outcome)> {
    input
        .lines()
        .map(|line| {
            (
                Shape::from(line.chars().next().unwrap()),
                Outcome::from(line.chars().nth(2).unwrap()),
            )
        })
        .collect::<Vec<_>>()
}

fn solve_part2(input: &str) -> (usize, Duration) {
    let timer = Instant::now();
    let games = parse_part2(input);
    let parse_duration = timer.elapsed();
    (
        games
            .iter()
            .map(|&(opponent_shape, desired_outcome)| {
                score(desired_outcome.manifest(opponent_shape), desired_outcome)
            })
            .sum::<usize>(),
        parse_duration,
    )
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let part1 = solve_part1(INPUT);
    let part2 = solve_part2(INPUT);
    (part1.0, part2.0, part1.1 + part2.1)
}
