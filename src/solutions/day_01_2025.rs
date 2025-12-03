use aor::solution::solution_main;

/// Day 1, 2025 | https://adventofcode.com/2025/day/1

fn parse(input: String) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let sign = if line.starts_with('L') { -1 } else { 1 };
            sign * line[1..].trim().parse::<i32>().unwrap()
        })
        .collect()
}

fn part1(input: String) -> anyhow::Result<String> {
    let instructions = parse(input);
    let mut angle = 50;
    let mut zeros = 0;
    for arc in instructions {
        angle = (angle + arc).rem_euclid(100);
        if angle == 0 {
            zeros += 1;
        }
    }
    Ok(zeros.to_string())
}

fn part2(input: String) -> anyhow::Result<String> {
    let instructions = parse(input);

    let mut angle = 50;
    let mut passed_zero = 0;

    for arc in instructions {
        let new_angle = angle + arc;
        passed_zero += if arc > 0 {
            new_angle.div_euclid(100)
        } else {
            (angle - 1).div_euclid(100) - (new_angle - 1).div_euclid(100)
        };
        angle = new_angle.rem_euclid(100);
    }

    Ok(passed_zero.to_string())
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
            .to_string();
        let expected_output = r#"3"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
            .to_string();
        let expected_output = r#"6"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
