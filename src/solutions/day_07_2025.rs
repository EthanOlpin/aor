#![allow(unused)]
#![feature(ascii_char)]
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    hash::Hash,
    ops::BitXor,
};

use anyhow::anyhow;
use aor::{
    grid::{Grid, Pos},
    parse,
    solution::solution_main,
};
use timing_macro::timing;

/// Day 7, 2025 | https://adventofcode.com/2025/day/7

fn part1(input: String) -> anyhow::Result<String> {
    let grid = Grid::<u8, 0>::from_string('\n', &input);
    let start = grid.position(|x| x == &b'S').unwrap();

    let mut frontier = VecDeque::from([start]);
    let mut visited = Grid::<bool, 0>::new(grid.width(), grid.height(), false);
    let mut split = HashSet::new();
    let mut split_count = 0;
    while let Some(pos) = frontier.pop_front() {
        if visited[pos] {
            continue;
        }
        visited[pos] = true;
        let mut row = pos.r;
        let col = pos.c;
        while (row as usize) < grid.height() {
            if grid[row as usize][col as usize] == b'^' {
                split.insert(Pos::new(row, col));
                if col > 0 {
                    let left = Pos::new(row, col - 1);
                    frontier.push_back(left);
                }
                if (col as usize) < grid.width() {
                    let right = Pos::new(row, col + 1);
                    frontier.push_back(right);
                };

                break;
            } else {
                row += 1
            }
        }
    }
    Ok(split.len().to_string())
}

fn part2(input: String) -> anyhow::Result<String> {
    let grid = Grid::<u8, 0>::from_string('\n', &input);
    let start = grid.position(|x| x == &b'S').unwrap();
    let mut memo = Grid::<usize, 0>::new(grid.width(), grid.height(), 0);
    let mut stack = Vec::with_capacity(grid.height() * grid.width());
    stack.push((start, false));
    let mut max_stack_size = grid.width() * grid.height();
    while let Some((pos, processed_chiildren)) = stack.pop() {
        if stack.len() > max_stack_size {
            max_stack_size = stack.len();
        }
        if pos.r as usize >= grid.height() || memo[pos] != 0 {
            continue;
        }
        if processed_chiildren {
            let result = if grid[pos] == b'^' {
                memo[pos.left()] + memo[pos.right()]
            } else {
                let down = pos.down();
                if down.r as usize >= grid.height() {
                    1
                } else {
                    memo[down]
                }
            };
            memo[pos] = result;
        } else {
            stack.push((pos, true));
            if grid[pos] == b'^' {
                stack.push((pos.right(), false));
                stack.push((pos.left(), false));
            } else {
                let down = pos.down();
                if (down.r as usize) < grid.height() {
                    stack.push((down, false));
                }
            }
        }
    }
    Ok(memo[start].to_string())
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#
            .to_string();
        let expected_output = r#"21"#.to_string();
        let result = part1(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_part2() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#
            .to_string();
        let expected_output = r#"40"#.to_string();
        let result = part2(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
