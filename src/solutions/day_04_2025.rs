#![feature(portable_simd)]

use aor::{grid::Grid, solution::solution_main};

/// Day 4, 2025 | https://adventofcode.com/2025/day/4

fn part1(input: String) -> anyhow::Result<String> {
    let grid = Grid::<u8, 1>::from_string_with_padding('\n', '.', &input);

    let mut accessible = 0;
    for pos in grid.row_scan_positions() {
        if grid[pos] != b'@' {
            continue;
        }
        let mut paper_count: u8 = 0;
        for neighbor in pos.neighbors8() {
            if grid[neighbor] == b'@' {
                paper_count += 1;
            }
        }
        if paper_count < 4 {
            accessible += 1;
        }
    }

    Ok(accessible.to_string())
}

fn part2(input: String) -> anyhow::Result<String> {
    let mut grid = Grid::<u8, 1>::from_string_with_padding('\n', '.', &input);
    let mut frontier = Vec::with_capacity(grid.width() * grid.height());
    let visited_mask = 0b10000;
    for pos in grid.row_scan_positions() {
        if grid[pos] == b'.' {
            continue;
        }
        let mut paper_count = 0;
        for neighbor_pos in pos.neighbors8() {
            let neighbor = grid[neighbor_pos];
            if neighbor != b'.' && neighbor != 0 {
                paper_count += 1;
            }
        }
        grid[pos] = paper_count;
        if paper_count < 4 {
            frontier.push(pos);
        }
    }

    let mut removed = 0;
    while let Some(pos) = frontier.pop() {
        if visited_mask & grid[pos] != 0 {
            continue;
        }
        grid[pos] |= visited_mask;
        removed += 1;

        for neighbor_pos in pos.neighbors8() {
            grid[neighbor_pos] -= 1;
            if grid[neighbor_pos] < 4 {
                frontier.push(neighbor_pos);
            }
        }
    }

    Ok(removed.to_string())
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
            .to_string();
        let expected_output = r#"13"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
            .to_string();
        let expected_output = r#"43"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
