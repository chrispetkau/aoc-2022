use self::input::{BOARDS, NUMBERS};
use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    num::ParseIntError,
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Number(usize);

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Turn(usize);

#[derive(Clone, Debug)]
struct Board(Vec<usize>);

impl Board {
    fn winning_turn(&self, number_to_turn: &HashMap<Number, Turn>) -> Result<Turn> {
        let turns = self
            .0
            .iter()
            .map(|&number| number_to_turn.get(&Number(number)).copied());
        let row_win_turns = (0..5).map(|row| {
            turns
                .clone()
                .skip(row * 5)
                .take(5)
                .max()
                .expect("5x5 board")
        });
        let column_win_turns = (0..5).map(|column| {
            turns
                .clone()
                .skip(column)
                .step_by(5)
                .take(5)
                .max()
                .expect("5x5 board")
        });
        row_win_turns
            .chain(column_win_turns)
            .min()
            .flatten()
            .ok_or_else(|| anyhow!("board number not found in number_to_turn map"))
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .flat_map(|line| line.split_whitespace().map(|n| n.parse::<usize>()))
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        Ok(Self(numbers))
    }
}

fn score(
    winning_turn_board: (Turn, &Board),
    numbers: &[usize],
    number_to_turn: &HashMap<Number, Turn>,
) -> Result<usize> {
    let (winning_turn, board) = winning_turn_board;
    let board_score = board
        .0
        .iter()
        .try_fold(0, |current, &number| -> Result<usize> {
            let number_score = if winning_turn
                < *number_to_turn
                    .get(&Number(number))
                    .ok_or_else(|| anyhow!("board number not found in number_to_turn map"))?
            {
                number
            } else {
                0
            };
            Ok(current + number_score)
        })?;
    Ok(numbers[winning_turn.0] * board_score)
}

fn solve_for(numbers: &[usize], boards: &str) -> Result<(usize, usize, Duration)> {
    let parse_start = Instant::now();
    let boards = boards
        .split("\n\n")
        .map(|board| board.parse::<Board>())
        .collect::<Result<Vec<Board>>>()?;
    let parse_duration = Instant::now() - parse_start;

    let number_to_turn = numbers
        .iter()
        .enumerate()
        .map(|(index, &number)| (Number(number), Turn(index)))
        .collect::<HashMap<Number, Turn>>();

    type Winner<'a> = Option<(Turn, &'a Board)>;
    let (first_winner, last_winner) = boards.iter().try_fold(
        (None, None),
        |(mut first_winner, mut last_winner), board| -> Result<(Winner, Winner)> {
            let winning_turn = board.winning_turn(&number_to_turn)?;
            if let Some((turn, _)) = first_winner {
                if winning_turn < turn {
                    first_winner = Some((winning_turn, board));
                }
            } else {
                first_winner = Some((winning_turn, board));
            }
            if let Some((turn, _)) = last_winner {
                if turn < winning_turn {
                    last_winner = Some((winning_turn, board));
                }
            } else {
                last_winner = Some((winning_turn, board));
            }
            Ok((first_winner, last_winner))
        },
    )?;

    let first_winning_score = score(
        first_winner.ok_or_else(|| anyhow!("no first winner"))?,
        numbers,
        &number_to_turn,
    )?;

    let last_winning_score = score(
        last_winner.ok_or_else(|| anyhow!("no last winner"))?,
        numbers,
        &number_to_turn,
    )?;

    Ok((first_winning_score, last_winning_score, parse_duration))
}

// TODO figure out why this is so much slower than Chris Ozeroff's solution
pub(crate) fn solve() -> (usize, usize, Duration) {
    match solve_for(&NUMBERS, BOARDS) {
        Ok(answer) => answer,
        Err(error) => {
            println!("Error solving day 4: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}
