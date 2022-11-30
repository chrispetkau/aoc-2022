fn find(
    input: &[usize],
    index: usize,
    select_bit_set_partition: impl Fn(usize, usize, usize) -> bool,
) -> usize {
    if input.len() == 1 {
        return input[0];
    }
    let mask = 1 << index;
    let (with_bit_set, with_bit_unset): (Vec<usize>, Vec<usize>) =
        input.iter().partition(|&n| (n & mask) != 0);
    let threshold = input.len() / 2;
    let selected_partition =
        &if select_bit_set_partition(threshold, with_bit_set.len(), with_bit_unset.len()) {
            with_bit_set
        } else {
            with_bit_unset
        };
    if index == 0 {
        assert_eq!(1, selected_partition.len());
        selected_partition[0]
    } else {
        find(selected_partition, index - 1, select_bit_set_partition)
    }
}

fn o2(bit_count: usize, input: &[usize]) -> usize {
    find(
        input,
        bit_count - 1,
        |threshold: usize, _with_bit_set: usize, with_bit_unset: usize| with_bit_unset <= threshold,
    )
}

fn co2(bit_count: usize, input: &[usize]) -> usize {
    find(
        input,
        bit_count - 1,
        |threshold: usize, with_bit_set: usize, with_bit_unset: usize| {
            with_bit_set != with_bit_unset && with_bit_set <= threshold
        },
    )
}

pub(super) fn solve_part2(bit_count: usize, input: &[usize]) -> usize {
    o2(bit_count, input) * co2(bit_count, input)
}
