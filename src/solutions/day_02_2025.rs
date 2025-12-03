use std::collections::HashSet;

use aor::solution::solution_main;

/// Day 2, 2025 | https://adventofcode.com/2025/day/2

fn parse_ranges(input: String) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start = start_str.trim().parse::<u64>().unwrap();
            let end = end_str.trim().parse::<u64>().unwrap();
            (start, end)
        })
        .collect()
}

fn part1(input: String) -> anyhow::Result<String> {
    let mut ranges = parse_ranges(input);
    ranges.sort_unstable();
    let lower_bound = |i: u32| -> u64 { 10u64.pow(i) + 10u64.pow(i / 2) };
    let upper_bound = |i: u32| -> u64 { 10u64.pow(i + 1) - 1 };

    let mut i = 1u32;
    let mut b_lo = lower_bound(i);
    let mut b_hi = upper_bound(i);
    let mut result = 0;
    for (a_lo, a_hi) in ranges {
        if a_hi < b_lo {
            continue;
        }
        while b_hi < a_lo {
            i += 2;
            b_hi = upper_bound(i);
        }
        b_lo = lower_bound(i);
        let inc = b_lo / (10u64.pow(i / 2));
        let lo = a_lo.max(b_lo).div_ceil(inc) * inc;
        for x in (lo..=b_hi.min(a_hi)).step_by(inc as usize) {
            result += x;
        }
    }
    Ok(result.to_string())
}

fn chunk_sizes(num_digits: u32) -> Vec<u32> {
    let mut mult = 1;
    let mut sizes = Vec::new();
    for l in (2..=num_digits / 2).rev() {
        if num_digits % l == 0 && mult % l != 0 {
            mult *= l;
            sizes.push(l);
        }
    }
    if sizes.is_empty() {
        sizes.push(1);
    }
    sizes
}

fn split_range_by_digits(lo: u64, hi: u64) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    let mut start = lo;
    while start <= hi {
        let num_digits = start.ilog10() + 1;
        let end_of_length = 10u64.pow(num_digits) - 1;
        let end = end_of_length.min(hi);
        result.push((start, end));
        start = end + 1;
    }
    result
}

fn part2(input: String) -> anyhow::Result<String> {
    let mut ranges = parse_ranges(input)
        .into_iter()
        .flat_map(|(lo, hi)| split_range_by_digits(lo, hi))
        .filter(|(lo, _)| lo >= &10)
        .collect::<Vec<_>>();

    ranges.sort_unstable();
    let mut invalid_ids = HashSet::new();

    for (lo, hi) in ranges {
        let num_digits = lo.ilog10() + 1;
        for chunk_size in chunk_sizes(num_digits) {
            let mut step = 0;
            for i in 0..(num_digits / chunk_size) {
                step += 10u64.pow(i * chunk_size);
            }
            let start = lo.div_ceil(step) * step;
            let end = (hi / step) * step;
            for x in (start..=end).step_by(step as usize) {
                invalid_ids.insert(x);
            }
        }
    }
    Ok(invalid_ids.iter().sum::<u64>().to_string())
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#
            .to_string();
        let expected_output = r#"1227775554"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#
            .to_string();
        let expected_output = r#"4174379265"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
