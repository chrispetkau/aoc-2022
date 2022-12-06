use self::input::INPUT;
use std::time::Duration;

mod input;

#[cfg(test)]
mod tests;

fn is_marker(m: &str, count: usize) -> bool {
    let mut a = m.chars();
    (0..count).all(|i| {
        let lhs = a.next().unwrap();
        m.chars().skip(i + 1).all(|rhs| lhs != rhs)
    })
}

fn solve_for_marker_length(input: &str, input_length: usize, marker_length: usize) -> usize {
    (0..input_length - marker_length)
        .find_map(|i| {
            let end = i + marker_length;
            let marker_candidate = &input[i..end];
            if is_marker(marker_candidate, marker_length) {
                Some(end)
            } else {
                None
            }
        })
        .unwrap()
}

fn solve_for(input: &str) -> (usize, usize, Duration) {
    let count = input.len();
    let part1 = solve_for_marker_length(input, count, 4);
    let part2 = solve_for_marker_length(input, count, 14);
    (part1, part2, Duration::new(0, 0))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
