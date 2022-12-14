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
        // End of a cycle
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

        // Start of a cycle.
        self.elapsed_instruction_cycles += 1;
        self.cycle += 1;

        self.instruction_index != instructions.len()
    }
}

fn solve_for(input: &str) -> Result<(usize, String, Duration)> {
    let timer = Instant::now();
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, ParseIntError>>()?;
    let parse_duration = timer.elapsed();

    const CRT_ROW_COUNT: usize = 6;
    const CRT_COLUMN_COUNT: usize = 40;

    let mut cpu = Cpu::default();
    let mut observation_cycle = 20;
    const OBSERVATION_PERIOD: usize = 40;
    let mut signal_strengths = vec![];
    let mut crt = vec!['.'; CRT_COLUMN_COUNT * CRT_ROW_COUNT];
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

        let crt_column = (cpu.cycle - 1) % CRT_COLUMN_COUNT;
        if cpu.x.wrapping_sub(1) <= crt_column && crt_column <= cpu.x.wrapping_add(1) {
            crt[cpu.cycle - 1] = '#';
        }
    }

    let part1 = signal_strengths.iter().sum();

    let mut part2 = String::new();
    for row in 0..CRT_ROW_COUNT {
        part2.extend(&crt[row * CRT_COLUMN_COUNT..(row + 1) * CRT_COLUMN_COUNT]);
        part2.push('\n');
    }
    println!("{part2}");

    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, String, Duration) {
    match solve_for(INPUT) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 10 error: {}", error);
            (0, "".to_owned(), Duration::new(0, 0))
        }
    }
}
