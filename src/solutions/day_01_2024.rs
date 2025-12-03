use aor::{parse, solution::solution_main, util};
use itertools::Itertools as _;

/// Day 1, 2024 | https://adventofcode.com/2024/day/1

fn part1(input: String) -> anyhow::Result<String> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = parse::uints::<u32>(&input).tuples().unzip();
    left.sort_unstable();
    right.sort_unstable();
    let mut result = 0;
    for (a, b) in left.into_iter().zip(right) {
        result += a.abs_diff(b);
    }
    Ok(result.to_string())
}

fn part2(input: String) -> anyhow::Result<String> {
    let (left, right): (Vec<_>, Vec<_>) = parse::uints::<u32>(&input).tuples().unzip();
    let counts = util::counts(&right);
    let mut result: u32 = 0;
    for a in left {
        if let Some(count) = counts.get(&a) {
            result += a * (*count as u32);
        }
    }
    Ok(result.to_string())
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#
            .to_string();
        let expected_output = r#"11"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#
            .to_string();
        let expected_output = r#"31"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
