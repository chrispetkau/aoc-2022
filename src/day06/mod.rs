use self::input::INPUT;
use std::time::Duration;

mod input;

#[cfg(test)]
mod tests;

fn is_marker(m: &str) -> bool {
    let mut a = m.chars();
    (0..4).all(|i| {
        let lhs = a.next().unwrap();
        m.chars().skip(i+1).all(|rhs| lhs != rhs)
    })
}

fn solve_for(input: &str) -> (usize, usize, Duration) {
    let count = input.len();
    let part1 = (0..count - 4)
        .find_map(|i| {
            let end = i + 4;
            let marker_candidate = &input[i..end];
            if is_marker(marker_candidate) {
                Some(end)
            } else {
                None
            }
        })
        .unwrap();
    (part1, 0, Duration::new(0, 0))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
