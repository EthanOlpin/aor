#![feature(portable_simd)]
#![feature(ascii_char)]

use aor::{solution::solution_main, timing::timing};

/// Day 6, 2025 | https://adventofcode.com/2025/day/6

fn part1(input: String) -> anyhow::Result<String> {
    let bytes = input.as_bytes();
    let mut lines: Vec<_> = bytes.split(|&c| c == b'\n').collect();
    let operator_spans = lines
        .pop()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|&(_, c)| !c.is_ascii_whitespace());

    let mut result = 0u64;
    for (start, &operator) in operator_spans.into_iter() {
        let operands = lines
            .iter()
            .map(|line| {
                line[start..]
                    .iter()
                    .skip_while(|&&c| !c.is_ascii_digit())
                    .take_while(|&c| c.is_ascii_digit())
                    .fold(0u64, |acc, &c| acc * 10 + (c - b'0') as u64)
            })
            .collect::<Vec<u64>>();

        result += match operator {
            b'+' => operands.iter().sum::<u64>(),
            b'*' => operands.iter().product(),
            _ => unreachable!(),
        };
    }
    Ok(result.to_string())
}

fn part2(input: String) -> anyhow::Result<String> {
    let bytes = input.into_bytes();
    let mut lines = bytes.split(|&c| c == b'\n').collect::<Vec<_>>();
    let operators = lines
        .pop()
        .unwrap()
        .iter()
        .rev()
        .filter(|&&c| !c.is_ascii_whitespace())
        .collect::<Vec<_>>();

    let mut transposed = vec![vec![0u8; lines.len()]; lines[0].len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, &c) in line.iter().rev().enumerate() {
            transposed[j][i] = c;
        }
    }

    let mut result = 0u64;
    let mut start = 0;
    for operator in operators {
        let mut acc = match operator {
            b'+' => 0u64,
            b'*' => 1u64,
            _ => unreachable!(),
        };
        for col in &transposed[start..] {
            start += 1;
            let operand = col
                .iter()
                .skip_while(|&&c| !c.is_ascii_digit())
                .take_while(|&c| c.is_ascii_digit())
                .fold(0u64, |acc, &c| acc * 10 + (c - b'0') as u64);
            if operand == 0 {
                break;
            }
            acc = match operator {
                b'+' => acc + operand,
                b'*' => acc * operand,
                _ => unreachable!(),
            };
        }

        result += acc;
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
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#
            .to_string();
        let expected_output = r#"4277556"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#
            .to_string();
        let expected_output = r#"3263827"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
