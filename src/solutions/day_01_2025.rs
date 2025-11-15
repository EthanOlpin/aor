#![allow(unused)]
use std::{fmt::Display, io::Read, time::SystemTime};

use anyhow::anyhow;
use aor::{
    parse,
    solution::{ExecResult, solution_main},
    timing,
};
use itertools::Itertools as _;

/// Day 1, 2025 | https://adventofcode.com/2025/day/1

fn part1(input: String) -> anyhow::Result<String> {
    let _ = input;
    Err(anyhow!("Not implemented"))
}

fn part2(input: String) -> anyhow::Result<String> {
    let _ = input;
    Err(anyhow!("Not implemented"))
}

fn main() -> anyhow::Result<()> {
    solution_main(part1, part2)
}
