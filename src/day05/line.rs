use super::point::Point;
use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub(super) struct Line((Point, Point));

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self((a, b))
    }

    pub fn a(&self) -> Point {
        self.0 .0
    }

    pub fn b(&self) -> Point {
        self.0 .1
    }

    pub fn horizontal(&self) -> bool {
        self.a().y() == self.b().y()
    }
    pub fn vertical(&self) -> bool {
        self.a().x() == self.b().x()
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ");
        let a = points
            .next()
            .ok_or_else(|| anyhow!("failed to get point a"))?;
        let b = points
            .next()
            .ok_or_else(|| anyhow!("failed to get point b"))?;
        Ok(Self::new(a.parse::<Point>()?, b.parse::<Point>()?))
    }
}
