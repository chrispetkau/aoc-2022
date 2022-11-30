use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct Span {
    ordinal: usize,
    start: usize,
    end: usize,
}

impl Span {
    /// If end < start, swap them.
    pub fn new(ordinal: usize, start: usize, last: usize) -> Self {
        if start < last {
            // TODO if you want to prevent this here, you need to do it in set_start() too
            // except we use the degenerate state as a terminating condition
            //assert_ne!(start, last + 1, "Degenerate span");
            Self {
                ordinal,
                start,
                end: last + 1,
            }
        } else {
            //assert_ne!(start + 1, last, "Degenerate span");
            Self {
                ordinal,
                start: last,
                end: start + 1,
            }
        }
    }

    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn last(&self) -> usize {
        self.end - 1
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn set_start(&mut self, start: usize) -> Result<()> {
        if start <= self.end {
            self.start = start;
            Ok(())
        } else {
            Err(anyhow!("Can't set start past end"))
        }
    }

    pub fn contains(&self, candidate: usize) -> bool {
        self.start <= candidate && candidate < self.end
    }
}
