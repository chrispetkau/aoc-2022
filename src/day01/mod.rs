use std::time::Duration;
use self::input::INPUT;

mod input;

#[cfg(test)]
mod tests;

fn count_increasing(input: &[usize], step_size: usize) -> usize {
    input
        .iter()
        .zip(input.iter().skip(step_size))
        .filter(|(a, b)| a < b)
        .count()
}

fn solve_for(input: &[usize]) -> (usize, usize) {
    (count_increasing(input, 1), count_increasing(input, 3))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let (part1, part2) = solve_for(&INPUT);
    (part1, part2, Duration::new(0, 0))
}
