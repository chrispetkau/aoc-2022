use self::input::INPUT;
use anyhow::{anyhow, Result};
use std::time::Duration;

mod input;

#[cfg(test)]
mod tests;

const DELIMETERS: [(char, char, usize, usize); 4] = [
    ('(', ')', 3, 1),
    ('[', ']', 57, 2),
    ('{', '}', 1197, 3),
    ('<', '>', 25137, 4),
];

#[derive(Debug)]
enum EvaluatedLine {
    Correct,
    Incomplete(Vec<char>),
    Illegal(char),
}

fn evaluate_line(line: &str) -> Result<EvaluatedLine> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if DELIMETERS.iter().any(|(open, _, _, _)| *open == c) {
            stack.push(c);
        } else if let Some((expected_open, _, _, _)) =
            DELIMETERS.iter().find(|(_, close, _, _)| *close == c)
        {
            if let Some(actual_open) = stack.pop() {
                if *expected_open != actual_open {
                    return Ok(EvaluatedLine::Illegal(c));
                }
            }
        } else {
            return Err(anyhow!("Not a delimeter: [{}]", c));
        }
    }
    // If the stack is not-empty, we have an incomplete line.
    Ok(if stack.is_empty() {
        EvaluatedLine::Correct
    } else {
        stack.reverse();
        EvaluatedLine::Incomplete(stack)
    })
}

/// Return score associated with close_delimeter
fn illegal_score(close_delimeter: char) -> Result<usize> {
    DELIMETERS
        .iter()
        .find_map(|(_, close, illegal_score, _)| {
            if *close == close_delimeter {
                Some(*illegal_score)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("Not a close delimeter: [{}]", close_delimeter))
}

fn completion_score(open_delimeter: char) -> Result<usize> {
    DELIMETERS
        .iter()
        .find_map(|(open, _, _, completion_score)| {
            if *open == open_delimeter {
                Some(*completion_score)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("Not a close delimeter: [{}]", open_delimeter))
}

fn completion_score_for_sequence(completion_sequence: &[char]) -> Result<usize> {
    completion_sequence
        .iter()
        .try_fold(0, |current, close_delimeter| {
            Ok((current * 5) + completion_score(*close_delimeter)?)
        })
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let evaluated_lines = input
        .lines()
        .map(evaluate_line)
        .collect::<Result<Vec<_>>>()?;

    let part1 = evaluated_lines
        .iter()
        .filter_map(|evaluated_line| {
            if let EvaluatedLine::Illegal(c) = evaluated_line {
                Some(illegal_score(*c))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    let mut completion_scores = evaluated_lines
        .iter()
        .filter_map(|evaluated_line| {
            if let EvaluatedLine::Incomplete(completion_sequence) = evaluated_line {
                Some(completion_score_for_sequence(completion_sequence))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>>>()?;
    completion_scores.sort_unstable();
    let part2 = completion_scores[completion_scores.len() / 2];

    Ok((part1, part2, Duration::new(0, 0)))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    match solve_for(INPUT) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 10 error: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}
