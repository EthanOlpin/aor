use anyhow::anyhow;
use regex::Regex;
use std::{
    fmt::Display,
    io::{BufRead, BufReader, Read, Write as _},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};

use crate::{event_date::EventDate, timing};

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_int())
    }
}

impl Part {
    pub fn to_int(&self) -> u8 {
        match self {
            Part::One => 1,
            Part::Two => 2,
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "1" {
            Ok(Part::One)
        } else if s == "2" {
            Ok(Part::Two)
        } else {
            Err(anyhow!("{s} is not a valid part, expected '1' or '2'"))
        }
    }
}

pub fn bin_name(date: &EventDate) -> String {
    format!("day_{:02}_{}", date.day, date.year)
}

pub fn solution_path(date: &EventDate) -> PathBuf {
    let bin_name = bin_name(date);
    PathBuf::from("src/solutions").join(format!("{bin_name}.rs"))
}

pub fn read_input() -> anyhow::Result<String> {
    let mut result = String::new();
    std::io::stdin().read_to_string(&mut result)?;
    Ok(result)
}

pub fn read_run_args() -> Result<Part, anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--part" || !"12".contains(&args[2]) {
        return Err(anyhow::anyhow!("Usage: <cmd> --part <1|2>"));
    };
    let part = if args[2] == "1" { Part::One } else { Part::Two };
    Ok(part)
}

pub enum ExecResult {
    Complete {
        answer: String,
        duration_secs: u64,
        duration_subsec_ns: u32,
    },
    Failed(String),
}

impl ExecResult {
    pub fn parse(stdout: &str, stderr: &str) -> anyhow::Result<Self> {
        let duration_re = Regex::new(r"Duration: (\d+)s, (\d+)ns")?;
        let caps = duration_re
            .captures(stderr)
            .ok_or(anyhow!("Execution duration not printed to stderr"))?;
        let duration_secs: u64 = caps[1].parse()?;
        let duration_subsec_ns: u32 = caps[2].parse()?;
        Ok(ExecResult::Complete {
            answer: stdout.trim().to_string(),
            duration_secs,
            duration_subsec_ns,
        })
    }
}

pub fn write_answer(answer: &str, duration: Duration) {
    println!("{}", answer);
    eprintln!(
        "Duration: {}s, {}ns",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}

type PartFn = fn(String) -> anyhow::Result<String>;

pub fn solution_main(part1: PartFn, part2: PartFn) -> anyhow::Result<()> {
    let part = read_run_args()?;
    let input = read_input()?;
    let solve = match part {
        Part::One => part1,
        Part::Two => part2,
    };
    let result = timing::it(|| solve(input))?;
    write_answer(&result.result?, result.duration);
    Ok(())
}

pub fn build(date: &EventDate, release_build: bool) -> anyhow::Result<()> {
    let bin_name = bin_name(date);
    let mut args = vec!["build", "--quiet", "--bin", &bin_name];
    if release_build {
        args.push("--release");
    }
    let build_output = std::process::Command::new("cargo").args(&args).output()?;
    if !build_output.status.success() {
        eprintln!(
            "Cargo build failed:\n{}",
            String::from_utf8_lossy(&build_output.stderr)
        );
        return Err(anyhow!("Failed to build receiver binary."));
    }
    Ok(())
}

pub fn exec(
    input: &str,
    part: Part,
    date: &EventDate,
    release_build: bool,
) -> anyhow::Result<ExecResult> {
    let bin_name = bin_name(&date);

    let target = if release_build { "release" } else { "debug" };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let binary_dir: PathBuf = [manifest_dir.as_str(), "target", target].iter().collect();
    let binary_path = binary_dir.join(&bin_name);

    let mut child = std::process::Command::new(&binary_path)
        .args(&["--part", &part.to_string()])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
    }

    let stderr_output = if let Some(mut stderr) = child.stderr.take() {
        let mut stderr_output = String::new();
        std::thread::spawn(move || {
            BufReader::new(&mut stderr)
                .lines()
                .for_each(|line| match line {
                    Ok(l) => {
                        if !l.starts_with("Duration: ") {
                            eprintln!("{}", l);
                        }
                        stderr_output.push_str(&l);
                    }
                    Err(err) => eprintln!("Error reading stderr line {err:?}"),
                });
            stderr_output
        })
    } else {
        return Err(anyhow!("Failed to capture stderr from child process"));
    };

    let output = child.wait_with_output()?;

    let stderr = stderr_output
        .join()
        .map_err(|err| anyhow!("Failed to join stderr listener: {err:?}"))?;

    if !output.status.success() {
        return Ok(ExecResult::Failed(stderr.into()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    ExecResult::parse(&stdout, &stderr)
}
