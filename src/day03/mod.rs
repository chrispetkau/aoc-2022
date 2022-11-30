use std::{cmp::Ordering, ops::Range, time::Duration};
use self::input::INPUT;

mod input;
#[cfg(test)]
mod tests;

fn solve_part1(bit_count: usize, input: &[usize]) -> usize {
    let threshold = input.len() / 2;
    let gamma = (0..bit_count)
        .filter_map(|index| {
            let mask = 1 << index;
            let match_count = input.iter().filter(|&&n| (n & mask) != 0).count();
            if threshold <= match_count {
                Some(mask)
            } else {
                None
            }
        })
        .fold(0, |current, mask| current | mask);
    let discarded_bit_count = usize::BITS as usize - bit_count;
    let epsilon = !gamma << discarded_bit_count >> discarded_bit_count;
    gamma * epsilon
}

/// Return the index of the element with a 1 in the bit at index or input.len() if none do.
fn find_one_start(input: &[usize], index: usize) -> usize {
    let mask = 1 << index;
    if let Err(one_start) = input.binary_search_by(|element| {
        if (element & mask) != 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }) {
        one_start
    } else {
        panic!();
    }
}

fn find_window(input: &[usize], index: usize, choose_most_common: bool) -> Range<usize> {
    if input.len() == 1 {
        return 0..1;
    }
    let one_start = find_one_start(input, index);
    if choose_most_common == (one_start <= input.len() / 2) {
        // 1-window
        one_start..input.len()
    } else {
        // 0-window
        0..one_start
    }
}

fn o2(input: &[usize], bit_count: usize) -> usize {
    let window = (0..bit_count).rev().fold(0..input.len(), |current, i| {
        let local_window = find_window(&input[current.clone()], i, true);
        local_window.start + current.start..local_window.end + current.start
    });
    assert!({
        let window_size = window.end - window.start;
        0 < window_size && window_size < 3
    });
    input[window.start]
}

fn co2(input: &[usize], bit_count: usize) -> usize {
    let window = (0..bit_count).rev().fold(0..input.len(), |current, i| {
        let local_window = find_window(&input[current.clone()], i, false);
        local_window.start + current.start..local_window.end + current.start
    });
    assert!({
        let window_size = window.end - window.start;
        0 < window_size && window_size < 3
    });
    input[window.end - 1]
}

fn solve_part2(bit_count: usize, input: &[usize]) -> usize {
    let mut sorted_input = Vec::new();
    sorted_input.resize(input.len(), 0);
    sorted_input.copy_from_slice(input);
    sorted_input.sort_unstable();
    let o2 = o2(&sorted_input, bit_count);
    let co2 = co2(&sorted_input, bit_count);
    o2 * co2
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    (
        solve_part1(INPUT.0, &INPUT.1),
        solve_part2(INPUT.0, &INPUT.1),
        Duration::new(0, 0),
    )
}
