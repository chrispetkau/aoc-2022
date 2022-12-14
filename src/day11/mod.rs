use self::input::INPUT;
use anyhow::{anyhow, Result};
use std::{
    mem::swap,
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(usize);

impl Deref for Item {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Item {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Item {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<usize>()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Worry(usize);

impl Deref for Worry {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Worry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Worry {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<usize>()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MonkeyIndex(usize);

impl Deref for MonkeyIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MonkeyIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for MonkeyIndex {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<usize>()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(Worry),
    Mul(Worry),
    Square,
}

impl FromStr for Operation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(':').nth(1).unwrap().split_whitespace().skip(3);
        match tokens.next().unwrap() {
            "+" => Ok(Self::Add(tokens.next().unwrap().parse::<Worry>()?)),
            "*" => Ok(if let Ok(worry) = tokens.next().unwrap().parse::<Worry>() {
                Self::Mul(worry)
            } else {
                Self::Square
            }),
            unhandled_op => Err(anyhow!("Unhandled op {unhandled_op}")),
        }
    }
}

impl Operation {
    fn apply(&self, item: &mut Item) {
        let current = **item;
        *item = match self {
            Operation::Add(worry) => Item(current + **worry),
            Operation::Mul(worry) => Item(current * **worry),
            Operation::Square => Item(current * current),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Test {
    divisible_by: usize,
    if_true: MonkeyIndex,
    if_false: MonkeyIndex,
}

impl FromStr for Test {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let divisible_by = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()?;
        let if_true = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<MonkeyIndex>()?;
        let if_false = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<MonkeyIndex>()?;
        Ok(Self {
            divisible_by,
            if_true,
            if_false,
        })
    }
}

impl Test {
    fn apply(&self, item: &Item) -> MonkeyIndex {
        if **item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let items = lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|item| item.trim().parse::<Item>())
            .collect::<Result<Vec<Item>, ParseIntError>>()?;
        let operation = lines.next().unwrap().parse::<Operation>()?;
        let mut test_text = String::new();
        (0..3).for_each(|_| {
            test_text.push_str(lines.next().unwrap());
            test_text.push('\n');
        });
        let test = test_text.parse::<Test>()?;
        Ok(Self {
            items,
            operation,
            test,
            inspection_count: 0,
        })
    }
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let mut monkeys = input
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>())
        .collect::<Result<Vec<Monkey>>>()?;
    let parse_duration = timer.elapsed();

    // println!("{monkeys:?}");

    (0..20).for_each(|_round| {
        (0..monkeys.len()).for_each(|monkey_index| {
            let monkey = &mut monkeys[monkey_index];
            let mut items = vec![];
            swap(&mut items, &mut monkey.items); // Take all items from the monkey.
            let operation = monkey.operation;
            let test = monkey.test;
            monkey.inspection_count += items.len();
            items.iter_mut().for_each(|item| {
                operation.apply(item);
                **item /= 3;
                let recipient = test.apply(item);
                monkeys[*recipient].items.push(*item);
            });
        });
    });

    monkeys.sort_unstable_by_key(|monkey| monkey.inspection_count);
    let part1 = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspection_count)
        .product();

    let part2 = 0;

    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT).unwrap()
}
