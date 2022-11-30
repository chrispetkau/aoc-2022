use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use self::input::INPUT;

mod input;

#[cfg(test)]
mod tests;

fn solve_for((polymer_template, pair_insertion_rules): (&str, &str)) -> (usize, usize, Duration) {
    let parse_start = Instant::now();
    let polymer_template = polymer_template
        .chars()
        .map(|c| c as u8 - b'A')
        .collect::<Vec<u8>>();
    let pair_insertion_rules = pair_insertion_rules
        .lines()
        .map(|line| {
            let mut rule = line.split(" -> ");
            let mut pair = rule.next().unwrap().chars();
            let element = rule.next().unwrap();
            (
                (
                    pair.next().unwrap() as u8 - b'A',
                    pair.next().unwrap() as u8 - b'A',
                ),
                element.chars().next().unwrap() as u8 - b'A',
            )
        })
        .collect::<HashMap<(u8, u8), u8>>();
    let parse_duration = parse_start.elapsed();

    let mut part1 = 0;
    const PART_1_STEP_COUNT: usize = 10;
    const PART_2_STEP_COUNT: usize = 40;

    let mut pairs_histogram = HashMap::new();
    polymer_template
        .iter()
        .copied()
        .zip(polymer_template.iter().copied().skip(1))
        .for_each(|pair| {
            *pairs_histogram.entry(pair).or_insert(0) += 1;
        });
    let last_element = *polymer_template.last().unwrap();
    (0..PART_2_STEP_COUNT).for_each(|step| {
        if step == PART_1_STEP_COUNT {
            part1 = analyze_polymer(&pairs_histogram, last_element);
        }
        let previous_pairs_histogram = pairs_histogram.clone();
        pairs_histogram.clear();
        previous_pairs_histogram.iter().for_each(|(pair, count)| {
            let inserted_element = pair_insertion_rules[pair];
            *pairs_histogram
                .entry((pair.0, inserted_element))
                .or_insert(0) += count;
            *pairs_histogram
                .entry((inserted_element, pair.1))
                .or_insert(0) += count;
        });
    });
    let part2 = analyze_polymer(&pairs_histogram, last_element);

    (part1, part2, parse_duration)
}

fn analyze_polymer(pairs_histogram: &HashMap<(u8, u8), usize>, last_element: u8) -> usize {
    const ELEMENT_TYPE_COUNT: usize = 26;
    let mut counts = vec![0; ELEMENT_TYPE_COUNT];
    pairs_histogram
        .iter()
        .for_each(|(&pair, count)| counts[pair.0 as usize] += count);
    counts[last_element as usize] += 1;
    let non_zero_counts = counts.iter().filter(|&&element| element != 0);
    let (min, max) = (
        non_zero_counts.clone().copied().min().unwrap(),
        non_zero_counts.copied().max().unwrap(),
    );
    max - min
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    solve_for(INPUT)
}
