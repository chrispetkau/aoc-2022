use input::INPUT;
use std::time::Duration;

mod input;

#[cfg(test)]
mod tests;

/// Given a digit and the known number of matching segments in a mystery digit,
/// return a mask representing the candidate digits.
/// Digit       : 9 8 7 6 5 4 3 2 1 0
/// Bit Index   : 9 8 7 6 5 4 3 2 1 0
fn get_candidates(digit: usize, matching_segment_count: usize) -> usize {
    const NONE: usize = 0b0;
    const ALL: usize = 0b1111111111;
    match digit {
        0 => match matching_segment_count {
            2 => 0b0000000010, // 1
            3 => 0b0010010000, // 4, 7
            4 => 0b0000101100, // 2, 3, 5
            5 => 0b1001000000, // 6,9
            6 => 0b0100000001, // 0, 8
            _ => ALL,
        },
        1 => match matching_segment_count {
            1 => 0b0001100100, // vec![2, 5, 6],
            2 => 0b1110011011, // vec![0, 1, 3, 4, 7, 8, 9],
            _ => ALL,
        },
        2 => match matching_segment_count {
            1 => 0b0000000010, // 1
            2 => 0b0010010000, //4, 7
            3 => 0b0000100000, // 5
            4 => 0b1001001001, // 0, 3, 6, 9
            5 => 0b0100000100, // 2,8
            _ => ALL,
        },
        3 => match matching_segment_count {
            1 => NONE,
            2 => 0b0000000010, // 1
            3 => 0b0010010000, //4, 7
            4 => 0b0001100101, // 0, 2, 5, 6
            5 => 0b1100001000, // 3, 8, 9
            _ => ALL,
        },
        4 => match matching_segment_count {
            2 => 0b0010000110, // vec![1, 2, 7],
            3 => 0b0001101001, // vec![0, 3, 5, 6],
            4 => 0b1100010000, // vec![4, 8, 9],
            _ => ALL,
        },
        5 => match matching_segment_count {
            1 => 0b0000000010, // 1
            2 => 0b0010000000, // 7
            3 => 0b0000011100, // 2, 3, 4
            4 => 0b0000000001, // 0,
            5 => 0b1101100000, // 5, 6, 8, 9
            _ => ALL,
        },
        6 => match matching_segment_count {
            1 => 0b0000000010, // 1
            2 => 0b0010000000, // 7
            4 => 0b0000011100, // 2,3,4
            5 => 0b1000100000, // 5,9
            6 => 0b0101000001, // 0,6, 8
            _ => ALL,
        },
        7 => match matching_segment_count {
            2 => 0b0001110110, // vec![1, 2, 4, 5, 6],
            3 => 0b1110001001, // vec![0, 3, 7, 8, 9],
            _ => ALL,
        },
        8 => match matching_segment_count {
            2 => 0b0000000010, // vec![1],
            3 => 0b0010000000, // vec![7],
            4 => 0b0000010000, // vec![4],
            5 => 0b0000101100, // vec![2, 3, 5],
            6 => 0b1001000001, // vec![0, 6, 9],
            7 => 0b0100000000, // 8
            _ => ALL,
        },
        9 => match matching_segment_count {
            2 => 0b0000000010, // 1
            3 => 0b0010000000, // 7
            4 => 0b0000010100, // 2, 4
            5 => 0b0001101001, // 0,3, 5, 6
            6 => 0b1100000000, // 8,9
            _ => ALL,
        },
        _ => ALL,
    }
}

fn make_masks(signals: &[&str]) -> Vec<usize> {
    const LOWER_A_ASCII: usize = 'a' as usize;
    signals
        .iter()
        .map(|signal| {
            signal.chars().fold(0b0, |current, c| {
                current | (1 << (c as usize - LOWER_A_ASCII))
            })
        })
        .collect::<Vec<_>>()
}

fn segment_count_to_digit(segment_count: usize) -> Option<usize> {
    match segment_count {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn count_set_bits(bit_count: usize, mask: usize) -> usize {
    (0..bit_count).fold(0, |current, bit_index| {
        if ((1 << bit_index) & mask) != 0 {
            current + 1
        } else {
            current
        }
    })
}

/// Assuming mask has a single bit set, return the index of that bit.
fn get_bit_index(bit_count: usize, mask: usize) -> usize {
    if let Some(bit_index) = (0..bit_count).find(|bit_index| mask == (1 << bit_index)) {
        bit_index
    } else {
        panic!()
    }
}

const WIRE_COUNT: usize = 7;
const DIGIT_COUNT: usize = 10;

fn decode(signals: &[&str], outputs: &[&str]) -> usize {
    let mut unordered_masks = make_masks(signals);

    let segment_counts = unordered_masks
        .iter()
        .map(|&mask| count_set_bits(WIRE_COUNT, mask))
        .collect::<Vec<usize>>();

    let mut mapped_digits: Vec<(usize, usize)> = unordered_masks
        .iter()
        .zip(
            segment_counts
                .iter()
                .map(|&segment_count| segment_count_to_digit(segment_count)),
        )
        .filter_map(|(&mask, digit)| digit.map(|digit| (digit, mask)))
        .collect::<Vec<_>>();

    unordered_masks.retain(|mask| {
        mapped_digits
            .iter()
            .all(|(_digit, mapped_mask)| mask != mapped_mask)
    });

    while !unordered_masks.is_empty() {
        let mask = unordered_masks.pop().unwrap();
        if let Err(digit) =
            mapped_digits
                .iter()
                .try_fold(0b1111111111, |candidates, (digit, mapped_mask)| {
                    let matching_segment_count = count_set_bits(WIRE_COUNT, mask & mapped_mask);
                    let new_candidates =
                        candidates & get_candidates(*digit, matching_segment_count);
                    if count_set_bits(DIGIT_COUNT, new_candidates) == 1 {
                        Err(get_bit_index(DIGIT_COUNT, new_candidates))
                    } else {
                        Ok(new_candidates)
                    }
                })
        {
            mapped_digits.push((digit, mask));
        } else {
            unordered_masks.insert(0, mask);
        }
    }
    assert_eq!(mapped_digits.len(), DIGIT_COUNT);

    let outputs = make_masks(outputs);
    const MULTIPLIERS: [usize; 4] = [1000, 100, 10, 1];
    MULTIPLIERS
        .iter()
        .zip(outputs.iter().map(|mask| {
            mapped_digits
                .iter()
                .find_map(|(digit, mapped_mask)| {
                    if mask == mapped_mask {
                        Some(digit)
                    } else {
                        None
                    }
                })
                .unwrap()
        }))
        .fold(0, |current, (multiplier, digit)| {
            current + multiplier * digit
        })
}

fn solve_for(input: &str) -> (usize, usize) {
    let signals_outputs = input.lines().flat_map(|line| line.split(" | "));
    let signal_sets = signals_outputs
        .clone()
        .step_by(2)
        .map(|signal| signal.split_whitespace().collect::<Vec<&str>>());
    let output_sets = signals_outputs
        .clone()
        .skip(1)
        .step_by(2)
        .map(|signal| signal.split_whitespace().collect::<Vec<&str>>());
    let part1 = output_sets
        .clone()
        .flat_map(|output_set| {
            output_set
                .iter()
                .filter_map(|output| segment_count_to_digit(output.len()))
                .collect::<Vec<_>>()
        })
        .count();
    let part2 = signal_sets
        .zip(output_sets)
        .map(|(signals, outputs)| decode(&signals, &outputs))
        .sum();
    (part1, part2)
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let (part1, part2) = solve_for(INPUT);
    (part1, part2, Duration::new(0, 0))
}
