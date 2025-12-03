use aor::solution::solution_main;

/// Day 3, 2025 | https://adventofcode.com/2025/day/3

fn digits(s: &str) -> Vec<u8> {
    s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn first_max(slice: &[u8]) -> (usize, &u8) {
    slice
        .iter()
        .enumerate()
        .max_by(|(ai, a), (bi, b)| a.cmp(b).then(bi.cmp(ai)))
        .unwrap()
}

fn max_combination(slice: &[u8], len: usize) -> u64 {
    let mut value = 0;
    let mut left_bound = 0;
    for min_tail_size in (0..len).rev() {
        let right_bound = slice.len() - min_tail_size;
        let (max_i, max_digit) = first_max(&slice[left_bound..right_bound]);
        value *= 10;
        value += *max_digit as u64;
        left_bound += max_i + 1;
    }
    value
}

fn part1(input: String) -> anyhow::Result<String> {
    let lines = input.lines().map(digits).collect::<Vec<_>>();
    let result = lines
        .iter()
        .map(|l| max_combination(l, 2))
        .sum::<u64>()
        .to_string();
    Ok(result)
}

fn part2(input: String) -> anyhow::Result<String> {
    let lines = input.lines().map(digits).collect::<Vec<_>>();
    let result = lines
        .iter()
        .map(|l| max_combination(l, 12))
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
