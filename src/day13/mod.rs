use anyhow::{anyhow, Result};
use input::INPUT;
use std::{
    cmp::Ordering,
    num::ParseIntError,
    str::FromStr,
    time::{Duration, Instant},
};

mod input;

#[cfg(test)]
mod tests;

/// An iterator over a string representing a list that can have lists as elements.
#[derive(Clone, Debug)]
struct ListIterator<'a> {
    s: &'a str,
    current: usize,
}

impl<'a> ListIterator<'a> {
    fn new(s: &'a str) -> Result<Self> {
        if !s.starts_with('[') || !s.ends_with(']') {
            return Err(anyhow!("String is not a list"));
        }
        Ok(Self { s, current: 1 })
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.s.len() - 1 {
            return None;
        }
        // From the current position, scan.
        // For every open bracket we encounter, we'll need to encounter a close bracket.
        // But if we find a comma, we're done.
        let mut open = 0;
        let offset = self.s[self.current..self.s.len() - 1]
            .chars()
            .take_while(|c| match c {
                '[' => {
                    open += 1;
                    true
                }
                ']' => {
                    open -= 1;
                    true
                }
                ',' => open != 0,
                _ => true,
            })
            .count();
        let new_current = self.current + offset;
        let element = &self.s[self.current..new_current];
        self.current = new_current;
        if self.s.chars().nth(self.current).unwrap() == ',' {
            self.current += 1;
        }
        Some(element)
    }
}

#[test]
fn list_iterator() {
    let s = "";
    assert!(ListIterator::new(s).is_err());

    let s = "[]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(0, i.count());

    let s = "[1]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(1, i.count());

    let s = "[[]]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(1, i.count());

    let s = "[[[]]]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(1, i.count());

    let s = "[7,7,7]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(3, i.count());

    let s = "[[7,7,7]]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(1, i.count());

    let s = "[[1],[2,3,4]]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(2, i.count());

    let s = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
    let i = ListIterator::new(s).unwrap();
    assert_eq!(4, i.count());
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Int(usize);

impl FromStr for Int {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<usize>()?))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct List(Vec<Packet>);

impl FromStr for List {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            ListIterator::new(s)?
                .map(|element| element.parse::<Packet>())
                .collect::<Result<Vec<Packet>>>()?,
        ))
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        for (lhs, rhs) in self.0.iter().zip(other.0.iter()) {
            let result = lhs.cmp(rhs);
            if result != Ordering::Equal {
                return result;
            }
        }
        self.0.len().cmp(&other.0.len())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(Int),
    List(List),
}

impl FromStr for Packet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(int) = s.parse::<Int>() {
            Ok(Self::Int(int))
        } else if let Ok(list) = s.parse::<List>() {
            Ok(Self::List(list))
        } else {
            Err(anyhow!("Not an integer and not a list"))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Int(lhs) => match other {
                Packet::Int(rhs) => lhs.cmp(rhs),
                Packet::List(rhs) => List(vec![Packet::Int(*lhs)]).cmp(rhs),
            },
            Packet::List(lhs) => match other {
                Packet::Int(rhs) => lhs.cmp(&List(vec![Packet::Int(*rhs)])),
                Packet::List(rhs) => lhs.cmp(rhs),
            },
        }
    }
}

fn solve_for(input: &str) -> Result<(usize, usize, Duration)> {
    let timer = Instant::now();
    let packet_pairs = input
        .split("\n\n")
        .map(|packet_pair| {
            let mut lines = packet_pair.lines();
            let a = lines.next().unwrap().parse::<List>()?;
            let b = lines.next().unwrap().parse::<List>()?;
            Ok((a, b))
        })
        .collect::<Result<Vec<_>>>()?;
    let parse_duration = timer.elapsed();

    // println!("{packet_pairs:?}");

    let part1 = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(index, (lhs, rhs))| if lhs <= rhs { Some(index + 1) } else { None })
        .sum();

    let divider_packets = ["[[2]]", "[[6]]"]
        .into_iter()
        .map(|s| s.parse::<List>())
        .collect::<Result<Vec<_>>>()?;
    let mut packets: Vec<&List> = vec![];
    let (lhs, rhs): (Vec<&List>, Vec<&List>) =
        packet_pairs.iter().map(|(lhs, rhs)| (lhs, rhs)).unzip();
    packets.extend(lhs);
    packets.extend(rhs);
    packets.extend(divider_packets.iter());
    packets.sort_unstable();

    let part2 = divider_packets
        .iter()
        .map(|divider_packet| {
            packets
                .iter()
                .enumerate()
                .find_map(|(index, packet)| {
                    if **packet == *divider_packet {
                        Some(index + 1)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .product();

    Ok((part1, part2, parse_duration))
}

pub(crate) fn solve() -> (usize, usize, Duration) {
    match solve_for(INPUT) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 13 error: {}", error);
            (0, 1, Duration::new(0, 0))
        }
    }
}
