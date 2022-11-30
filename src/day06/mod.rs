use std::{iter, time::Duration};
use self::input::INPUT;

mod input;

#[cfg(test)]
mod tests;

fn solve_for(input: &[usize], day_counts: &[usize]) -> Vec<usize> {
    const GESTATION_DURATION: usize = 7;
    const MATURATION_DURATION: usize = 2;
    const TIMER_COUNT: usize = GESTATION_DURATION + MATURATION_DURATION;
    let mut timers: [usize; GESTATION_DURATION + MATURATION_DURATION] = [0; TIMER_COUNT];
    input.iter().for_each(|&timer| timers[timer] += 1);
    let mut zero_day = 0;
    let day_markers = iter::once(&0).chain(day_counts.iter());
    day_markers
        .clone()
        .zip(day_markers.clone().skip(1))
        .map(|(&start, &end)| {
            (start..end).for_each(|_| {
                let baby_fish_count = timers[zero_day];

                // Ascension to adulthood.
                timers[zero_day] += timers[GESTATION_DURATION];

                // Adolescents mature.
                (0..MATURATION_DURATION - 1).for_each(|i| {
                    timers[GESTATION_DURATION + i] = timers[GESTATION_DURATION + i + 1];
                });

                // New fish are born.
                timers[TIMER_COUNT - 1] = baby_fish_count;

                // Advance to next day.
                zero_day = (zero_day + 1) % GESTATION_DURATION;
            });
            timers.iter().sum()
        })
        .collect::<Vec<usize>>()
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    let solutions = solve_for(&INPUT, &[80, 256]);
    (solutions[0], solutions[1], Duration::new(0,0))
}
