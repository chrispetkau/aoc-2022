use std::time::Duration;
use self::input::INPUT;

mod input;

#[cfg(test)]
mod tests;

fn fuel_cost_part1(distance: i32) -> i32 {
    distance
}

fn fuel_cost_part2(distance: i32) -> i32 {
    (0..distance).sum::<i32>() + distance
}

fn solve_for(input: &[i32]) -> (i32, i32) {
    assert_ne!(input.len(), 0);
    let (min, max): (i32, i32) = input.iter().fold(
        (input[0], input[0]),
        |(current_min, current_max), &position| {
            (current_min.min(position), current_max.max(position))
        },
    );
    let part1 = (min..=max)
        .map(|alignment_candidate| {
            input.iter().fold(0, |current, &position| {
                current + fuel_cost_part1((position - alignment_candidate).abs())
            })
        })
        .min()
        .unwrap();
    let part2 = (min..=max)
        .map(|alignment_candidate| {
            input.iter().fold(0, |current, &position| {
                current + fuel_cost_part2((position - alignment_candidate).abs())
            })
        })
        .min()
        .unwrap();
    (part1, part2)
}

pub(crate) fn solve() -> (i32, i32, Duration) {
    let (part1, part2) = solve_for(&INPUT);
    (part1, part2, Duration::new(0,0))
}
