#![feature(range_into_bounds)]

use aor::solution::solution_main;
use itertools::Itertools;

/// Day 5, 2025 | https://adventofcode.com/2025/day/5

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (head, tail) = input.split_once("\n\n").unwrap();
    let ranges = head
        .split(['\n', '-'])
        .map(|x| x.parse::<u64>().unwrap())
        .tuples()
        .sorted_unstable()
        .collect();

    let values = tail
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .sorted_unstable()
        .collect();

    (ranges, values)
}

fn part1(input: String) -> anyhow::Result<String> {
    let (ranges, values) = parse(&input);
    let mut count = 0;
    let mut ranges = ranges.into_iter();
    let mut range = ranges.next();
    for value in values {
        while let Some((lo, hi)) = range {
            if value < lo {
                break;
            } else if value <= hi {
                count += 1;
                break;
            } else {
                range = ranges.next();
            }
        }
    }
    Ok(count.to_string())
}

fn part2(input: String) -> anyhow::Result<String> {
    let (ranges, _) = parse(&input);
    let mut it = ranges.into_iter();
    let mut acc = it.next().unwrap();
    let mut total_size = 0;
    for (lo, hi) in it {
        let (acc_lo, acc_hi) = acc;
        if hi < acc_lo || lo > acc_hi {
            total_size += acc_hi - acc_lo + 1;
            acc = (lo, hi);
        } else {
            acc = (acc_lo.min(lo), acc_hi.max(hi));
        }
    }

    let (acc_lo, acc_hi) = acc;
    total_size += acc_hi - acc_lo + 1;
    Ok(total_size.to_string())
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
        .to_string();
        let expected_output = r#"3"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
        .to_string();
        let expected_output = r#"14"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
