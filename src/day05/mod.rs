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
    // Remove the stack numbers, invert the stacks, and remove the empty elements.
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
    let mut part1_stacks = Vec::from(stacks);
    let mut part2_stacks = part1_stacks.clone(); // TODO should be able to use stacks directly

    steps.iter().for_each(|step| {
        (0..step.count).for_each(|_| {
            let c = part1_stacks[step.from].pop().unwrap();
            part1_stacks[step.to].push(c);
        });
    });
    let part1 = part1_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    steps.iter().for_each(|step| {
        let popped = part2_stacks[step.from]
            .iter()
            .copied()
            .rev()
            .take(step.count)
            .rev()
            .collect::<Vec<_>>();
        part2_stacks[step.to].extend(popped);

        let from = &mut part2_stacks[step.from];
        let from_len = from.len();
        from.resize(from_len - step.count, ' ');
    });
    let part2 = part2_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    (part1, part2)
}

pub(crate) fn solve() -> (String, String, Duration) {
    let timer = Instant::now();
    let parsed = parse(INPUT);
    let parse_duration = Instant::now() - timer;
    let (part1, part2) = match parsed {
        Ok((mut stacks, steps)) => solve_for(&mut stacks, &steps),
        Err(error) => {
            println!("day 5 error: {}", error);
            ("".to_owned(), "".to_owned())
        }
    };
    (part1, part2, parse_duration)
}
