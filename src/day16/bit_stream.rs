use anyhow::{anyhow, Result};
use std::str::FromStr;

// TODO really, the data should be kept separate from the iterator, but I don't think I want an actual Iterator
// implementation because when I deref the iterator, I want to take N bits as a u32
pub(crate) struct BitStream {
    bytes: Vec<u8>,
    bit_index: usize,
}

const HEX_RADIX: u32 = 16;
const BITS_PER_HEX_DIGIT: usize = 4;

impl FromStr for BitStream {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .lines()
            .flat_map(|line| {
                line.chars()
                    .step_by(2)
                    .zip(line.chars().skip(1).step_by(2))
                    .map(|(a, b)| match a.to_digit(16) {
                        Some(a) => {
                            let b = b
                                .to_digit(HEX_RADIX)
                                .ok_or_else(|| anyhow!("Failed to convert to hex"))?;
                            Ok(((a << BITS_PER_HEX_DIGIT) | b) as u8)
                        }
                        None => Err(anyhow!("Failed to convert to hex")),
                    })
            })
            .collect::<Result<Vec<u8>>>()?;
        Ok(Self {
            bytes,
            bit_index: 0,
        })
    }
}

const BITS_PER_BYTE: usize = 8;
impl BitStream {
    pub fn index(&self) -> usize {
        self.bit_index
    }

    /// Take the next 'bit_count' bits from the stream and return them interpreted as a T.
    pub fn take(&mut self, bit_count: usize) -> u32 {
        assert!(bit_count <= u32::BITS as usize);

        let current_byte_index = self.bit_index / BITS_PER_BYTE;
        let current_bit_index = self.bit_index % BITS_PER_BYTE;
        self.bit_index += bit_count;
        let next_byte_index = self.bit_index / BITS_PER_BYTE;
        let next_bit_index = self.bit_index % BITS_PER_BYTE;

        // Push the owned byte specified by byte_index onto the left side of bits.
        let push_byte = |byte_index: usize, bits: u32| -> u32 {
            let mut byte = self.bytes[byte_index];
            if byte_index == current_byte_index {
                byte <<= current_bit_index;
                byte >>= current_bit_index;
            }
            (bits << BITS_PER_BYTE) | byte as u32
        };

        let mut bits = (current_byte_index..next_byte_index)
            .fold(0, |current, byte_index| push_byte(byte_index, current));

        if next_bit_index != 0 {
            bits = push_byte(next_byte_index, bits) >> (BITS_PER_BYTE - next_bit_index);
        }

        bits
    }
}

#[test]
fn parse() {
    const INPUT: &str = "D2FE28";
    let mut bit_stream = INPUT.parse::<BitStream>().unwrap();

    INPUT.chars().for_each(|c| {
        assert_eq!(
            c.to_digit(HEX_RADIX).unwrap(),
            bit_stream.take(BITS_PER_HEX_DIGIT),
            "Failed to extract character: {}",
            c
        );
    });

    bit_stream.bit_index = 0; // Reset the BitStream before reading it again.
    assert_eq!(0b110100101111111000101000, bit_stream.take(24));
}
