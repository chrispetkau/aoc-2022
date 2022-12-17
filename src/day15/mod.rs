use anyhow::Result;
use std::{
    cmp::Ordering,
    num::ParseIntError,
    str::FromStr,
    time::{Duration, Instant},
};

use input::INPUT;

mod input;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_index = s.match_indices("x=").next().unwrap().0;
        let mut tokens = s[x_index..].split(',');
        let x = tokens.next().unwrap()[2..].parse::<i32>()?;
        let y = tokens.next().unwrap()[3..].parse::<i32>()?;
        Ok(Self { x, y })
    }
}

impl Point {
    fn manhattan_distance(&self, rhs: &Point) -> usize {
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()) as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sensor(Point);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Beacon(Point);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EffectiveSensor {
    sensor: Sensor,
    range: usize,
}

impl EffectiveSensor {
    fn span_at(&self, y: i32) -> Option<Span> {
        // When the sensor is on the same row as the query row, the x-range has a width equal to 2*radius+1
        // (the +1 is for the sensor itself).
        // When the sensor is 1 row away, the width is 2 less...
        let y_delta = (y - self.sensor.0.y).abs();
        let x_range = self.range as i32 - y_delta;
        if x_range < 0 {
            None
        } else {
            Some(Span {
                start: self.sensor.0.x - x_range,
                length: x_range as usize * 2,
            })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Span {
    start: i32,
    length: usize,
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl Span {
    fn end(&self) -> i32 {
        self.start + self.length as i32
    }

    fn merge(&self, rhs: &Self) -> Option<Self> {
        let end = self.end();
        if end >= rhs.start {
            Some(Self {
                start: self.start,
                length: (end.max(rhs.end()) - self.start) as usize,
            })
        } else {
            None
        }
    }
}

fn solve_for(input: &str, y: i32) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let effective_sensors = input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let sensor = parts.next().unwrap();
            let beacon = parts.next().unwrap();
            let sensor = sensor.parse::<Point>()?;
            let sensor = Sensor(sensor);
            let beacon = beacon.parse::<Point>()?;
            Ok(EffectiveSensor {
                sensor,
                range: sensor.0.manhattan_distance(&beacon),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let parse_duration = timer.elapsed();

    // Beacons serve to define a sensor's effective range.
    // Given a row, which sensors intersect it?
    // Given the set of sensors intersecting a row, what is the x-range of the intersection?

    let mut spans = effective_sensors
        .iter()
        .filter_map(|sensor| sensor.span_at(y))
        .collect::<Vec<Span>>();
    spans.sort_unstable();
    let mut i = 0;
    while i + 1 < spans.len() {
        if let Some(merge) = spans[i].merge(&spans[i + 1]) {
            spans[i] = merge;
            spans.remove(i + 1);
        } else {
            i += 1;
        }
    }

    let part1 = spans.iter().map(|span| span.length).sum::<usize>();
    let part2 = 1;

    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    match solve_for(INPUT, 2_000_000) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 15 error: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}
