use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) struct Point((usize, usize));

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { 0: (x, y) }
    }

    pub fn x(&self) -> usize {
        self.0 .0
    }

    pub fn y(&self) -> usize {
        self.0 .1
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coordinates = s.split(',');
        let x = coordinates
            .next()
            .ok_or_else(|| anyhow!("failed to get x"))?;
        let y = coordinates
            .next()
            .ok_or_else(|| anyhow!("failed to get y"))?;
        Ok(Self::new(x.parse::<usize>()?, y.parse::<usize>()?))
    }
}
