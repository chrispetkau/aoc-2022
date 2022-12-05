use self::input::INPUT;
use anyhow::Result;
use std::{
    num::ParseIntError,
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

type Stack = Vec<char>;

#[derive(Debug)]
pub(crate) struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ').skip(1).step_by(2);
        Ok(Self {
            count: tokens.next().unwrap().parse::<usize>()?,
            from: tokens.next().unwrap().parse::<usize>()? - 1,
            to: tokens.next().unwrap().parse::<usize>()? - 1,
        })
    }
}

pub(crate) fn parse(input: &str) -> Result<(Vec<Stack>, Vec<Step>)> {
    let mut sections = input.split("\n\n");
    let initial_stack_state = sections.next().unwrap();
    let stack_count = initial_stack_state
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .count();
    let mut stacks = vec![Vec::new(); stack_count];
    initial_stack_state.lines().for_each(|line| {
        let crates = line.chars().skip(1).step_by(4);
        stacks
            .iter_mut()
            .zip(crates)
            .for_each(|(stack, stacked_crate)| stack.push(stacked_crate));
    });
    // Remove the stack numbers and invert the stacks.
    stacks.iter_mut().for_each(|stack| {
        stack.pop();
        stack.reverse();
        stack.retain(|&c| c != ' ');
    });
    let steps = sections.next().unwrap();
    let steps = steps
        .lines()
        .map(|line| -> Result<Step, ParseIntError> { line.parse::<Step>() })
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    Ok((stacks, steps))
}

pub(crate) fn solve_for(stacks: &mut [Stack], steps: &[Step]) -> (String, String) {
    steps.iter().for_each(|step| {
        (0..step.count).for_each(|_| {
            let c = stacks[step.from].pop().unwrap();
            stacks[step.to].push(c);
        });
    });
    let part1 = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();
    (part1, "".to_owned())
}

pub(crate) fn solve() -> (String, String, Duration) {
    let timer = Instant::now();
    let parsed = parse(INPUT);
    println!("{:#?}", parsed.as_ref().unwrap().0);
    let parse_duration = Instant::now() - timer;
    let (part1, part2) = match parsed {
        Ok((mut stacks, steps)) => {
            solve_for(&mut stacks, &steps)
            //span_solution::solve_for(lines)
        }
        Err(error) => {
            println!("day 5 error: {}", error);
            ("".to_owned(), "".to_owned())
        }
    };
    (part1, part2, parse_duration)
}
