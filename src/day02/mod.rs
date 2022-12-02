use self::input::INPUT;
use std::time::Duration;

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

impl From<Outcome> for usize {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
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

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let opponent_shape = Shape::from(line.chars().nth(0).unwrap());
            let my_shape = Shape::from(line.chars().nth(2).unwrap());
            usize::from(my_shape) + usize::from(outcome(opponent_shape, my_shape))
        })
        .sum::<usize>()
}

fn solve_part2(input: &str) -> usize {
    0
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    (
        solve_part1(&INPUT),
        solve_part2(&INPUT),
        Duration::new(0, 0),
    )
}
