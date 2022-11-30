use std::time::Duration;
use self::input::INPUT;

mod input;

#[cfg(test)]
mod tests;

enum Direction {
    Forward,
    Down,
    Up,
}

struct Location {
    position: usize,
    depth: usize,
    aim: usize,
}

fn solve_part1(input: &[(Direction, usize)]) -> usize {
    let mut location = Location {
        position: 0,
        depth: 0,
        aim: 0,
    };
    input
        .iter()
        .for_each(|(direction, distance)| match direction {
            Direction::Forward => location.position += distance,
            Direction::Down => location.depth += distance,
            Direction::Up => location.depth -= distance,
        });
    location.position * location.depth
}

fn solve_part2(input: &[(Direction, usize)]) -> usize {
    let mut location = Location {
        position: 0,
        depth: 0,
        aim: 0,
    };
    input.iter().for_each(|(direction, units)| match direction {
        Direction::Forward => {
            location.position += units;
            location.depth += location.aim * units;
        }
        Direction::Down => location.aim += units,
        Direction::Up => location.aim -= units,
    });
    location.position * location.depth
}

pub(crate) fn solve() -> (usize, usize, Duration){
    (solve_part1(&INPUT), solve_part2(&INPUT), Duration::new(0,0))
}