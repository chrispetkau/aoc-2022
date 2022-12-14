use self::input::INPUT;
use anyhow::Result;
use std::{
    num::ParseIntError,
    ops::{Add, Sub},
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    AddX(i32),
    NoOp,
}

impl Instruction {
    fn duration(&self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::NoOp => 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        match tokens.next().unwrap() {
            "addx" => Ok(Self::AddX(tokens.next().unwrap().parse::<i32>()?)),
            "noop" => Ok(Self::NoOp),
            _ => panic!(),
        }
    }
}

struct Cpu {
    x: usize,
    cycle: usize,
    instructions: Option<Vec<Instruction>>,
    instruction_index: usize,
    elapsed_instruction_cycles: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            cycle: 0,
            instructions: None,
            instruction_index: 0,
            elapsed_instruction_cycles: 0,
        }
    }
}

impl Cpu {
    fn signal_strength(&self) -> usize {
        self.cycle.saturating_mul(self.x)
    }

    fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = Some(instructions);
        self.x = 1;
        self.instruction_index = 0;
        self.elapsed_instruction_cycles = 0;
    }

    fn tick(&mut self) -> bool {
        let instructions = self.instructions.as_ref().unwrap();
        let instruction = &instructions[self.instruction_index];
        if self.elapsed_instruction_cycles == instruction.duration() {
            match instruction {
                Instruction::AddX(v) => {
                    self.x = ((self.x as i32) + *v) as usize;
                }
                Instruction::NoOp => {}
            }
            self.instruction_index += 1;
            self.elapsed_instruction_cycles = 0;
        }
        self.elapsed_instruction_cycles += 1;
        self.cycle += 1;
        self.instruction_index != instructions.len()
    }
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, ParseIntError>>()?;
    let parse_duration = timer.elapsed();

    let mut cpu = Cpu::default();
    let mut observation_cycle = 20;
    const OBSERVATION_PERIOD: usize = 40;
    let mut signal_strengths = vec![];
    cpu.load(instructions);
    loop {
        if cpu.cycle == observation_cycle {
            signal_strengths.push(cpu.signal_strength());
            observation_cycle += OBSERVATION_PERIOD;
        }
        
        // println!("[{}] x = {}, signal = {}", cpu.cycle, cpu.x, cpu.signal_strength());

        if !cpu.tick() {
            break;
        }
    }

    let part1 = signal_strengths.iter().sum();
    let part2 = 1;

    Ok((part1, part2, parse_duration))
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
