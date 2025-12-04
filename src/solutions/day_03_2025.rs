#![feature(portable_simd)]
use aor::solution::solution_main;
use std::simd::prelude::*;

/// Day 3, 2025 | https://adventofcode.com/2025/day/3
fn first_max(slice: &[u8]) -> (usize, u8) {
    if slice.len() == 1 {
        return (0, slice[0]);
    }

    let mut best_max = 0u8;
    let mut best_pos = 0usize;

    for (chunk_idx, chunk) in slice.chunks(64).enumerate() {
        let v = Simd::<u8, 64>::load_or_default(chunk);

        let chunk_max = v.reduce_max();

        if chunk_max > best_max {
            best_max = chunk_max;
            let mask = v.simd_eq(Simd::splat(chunk_max)).to_bitmask();
            best_pos = chunk_idx * 64 + mask.trailing_zeros() as usize;
        }
    }

    (best_pos, best_max)
}

fn max_combination(slice: &[u8], size: usize) -> u64 {
    let mut value = 0u64;
    let mut left_bound = 0;

    for min_tail_size in (0..size).rev() {
        let right_bound = slice.len() - min_tail_size;
        let (max_i, max_byte) = first_max(&slice[left_bound..right_bound]);
        value = value * 10 + (max_byte - b'0') as u64;
        left_bound += max_i + 1;
    }
    value
}

fn part1(input: String) -> anyhow::Result<String> {
    let result = input
        .lines()
        .map(|l| max_combination(l.as_bytes(), 2))
        .sum::<u64>()
        .to_string();
    Ok(result)
}

fn part2(input: String) -> anyhow::Result<String> {
    let result = input
        .lines()
        .map(|l| max_combination(l.as_bytes(), 12))
        .sum::<u64>()
        .to_string();
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#
            .to_string();
        let expected_output = r#"357"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#
            .to_string();
        let expected_output = r#"3121910778619"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
