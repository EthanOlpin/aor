#![allow(unused)]
use std::{fmt::Display, io::Read, time::SystemTime};

use anyhow::anyhow;
use aor::{
    parse, solution::{ExecResult, solution_main}, timing};
use itertools::Itertools as _;

/// Day {{d}}, {{yyyy}} | https://adventofcode.com/{{yyyy}}/day/{{d}}

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
