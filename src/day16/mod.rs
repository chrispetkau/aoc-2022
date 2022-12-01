use self::{bit_stream::BitStream, input::INPUT};
use anyhow::{anyhow, Result};
use std::time::Duration;

mod input;

#[cfg(test)]
mod tests;

mod bit_stream;

const SUM: u32 = 0;
const PRODUCT: u32 = 1;
const MINIMUM: u32 = 2;
const MAXIMUM: u32 = 3;
const LITERAL: u32 = 4;
const GREATER_THAN: u32 = 5;
const LESS_THAN: u32 = 6;
const EQUAL_TO: u32 = 7;

type Value = u64;

fn evaluate_packet(operator: u32, operands: &[Value]) -> Result<Value> {
    match operator {
        SUM => Ok(operands.iter().sum()),
        PRODUCT => Ok(operands.iter().product()),
        MINIMUM => operands
            .iter()
            .copied()
            .min()
            .ok_or_else(|| anyhow!("No operands")),
        MAXIMUM => operands
            .iter()
            .copied()
            .max()
            .ok_or_else(|| anyhow!("No operands")),
        GREATER_THAN => Ok(u64::from(operands[0] > operands[1])),
        LESS_THAN => Ok(u64::from(operands[0] < operands[1])),
        EQUAL_TO => Ok(u64::from(operands[0] == operands[1])),
        bad_operator => Err(anyhow!("Bad operator: {}", bad_operator)),
    }
}

const VERSION_LENGTH: usize = 3;
const TYPE_ID_LENGTH: usize = 3;
const LITERAL_GROUP_LENGTH: usize = 4;
const PACKET_SIZE_LENGTH: usize = 15;
const SUB_PACKET_COUNT_LENGTH: usize = 11;
const LENGTH_TYPE_ID_LENGTH: usize = 1;
const LENGTH_TYPE_PACKET_LENGTH: u32 = 0;
const LENGTH_TYPE_SUB_PACKET_COUNT: u32 = 1;

fn read_next_packet(bit_stream: &mut BitStream, version_sum: &mut u32) -> Result<Value> {
    let version = bit_stream.take(VERSION_LENGTH);
    *version_sum += version;
    let type_id = bit_stream.take(TYPE_ID_LENGTH);
    match type_id {
        LITERAL => {
            let mut literal: Value = 0;
            let mut first_group = true;
            loop {
                let last_group = bit_stream.take(1) == 0;
                let group = bit_stream.take(LITERAL_GROUP_LENGTH);
                if first_group {
                    first_group = false;
                } else {
                    literal <<= LITERAL_GROUP_LENGTH;
                }
                literal |= group as Value;
                if last_group {
                    break Ok(literal);
                }
            }
        }
        operator => {
            let length_type_id = bit_stream.take(LENGTH_TYPE_ID_LENGTH);
            let operands = match length_type_id {
                LENGTH_TYPE_PACKET_LENGTH => {
                    let packet_size = bit_stream.take(PACKET_SIZE_LENGTH);
                    let mut operands = Vec::new();
                    let mut bits_consumed = 0;
                    while bits_consumed < packet_size {
                        let current_bit_index = bit_stream.index();
                        operands.push(read_next_packet(bit_stream, version_sum)?);
                        bits_consumed += (bit_stream.index() - current_bit_index) as u32;
                    }
                    if bits_consumed != packet_size {
                        return Err(anyhow!(
                            "Failed to consume all bits of packet: {}/{} consumed",
                            bits_consumed,
                            packet_size
                        ));
                    }
                    operands
                }
                LENGTH_TYPE_SUB_PACKET_COUNT => {
                    let sub_packet_count = bit_stream.take(SUB_PACKET_COUNT_LENGTH);
                    (0..sub_packet_count)
                        .map(|_| read_next_packet(bit_stream, version_sum))
                        .collect::<Result<Vec<Value>>>()?
                }
                bad_length_type => return Err(anyhow!("Bad length type: {}", bad_length_type)),
            };
            evaluate_packet(operator, &operands)
        }
    }
}

fn solve_for(input: &str) -> Result<(u32, Value, Duration)> {
    let mut bit_stream = input.parse::<BitStream>()?;
    let mut part1 = 0;
    let part2 = read_next_packet(&mut bit_stream, &mut part1)?;
    Ok((part1, part2, Duration::new(0, 0)))
}

pub(crate) fn solve() -> (u32, Value, Duration) {
    match solve_for(INPUT) {
        Ok(solution) => solution,
        Err(error) => {
            println!("day 16 error: {}", error);
            (0, 0, Duration::new(0, 0))
        }
    }
}
