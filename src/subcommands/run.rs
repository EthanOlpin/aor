use std::{fs, io::IsTerminal, path::PathBuf, time::Duration};

use crate::{
    aoc_client,
    event_date::EventDate,
    solution::{self, ExecResult},
};

fn get_input(date: &EventDate, override_path: Option<PathBuf>) -> anyhow::Result<String> {
    if let Some(path) = override_path {
        return Ok(fs::read_to_string(path)?);
    }
    aoc_client::get_input(date)
}

pub fn run(
    day: Option<u8>,
    year: Option<u16>,
    parts: &[solution::Part],
    input_override_path: Option<PathBuf>,
    release_build: bool,
) -> anyhow::Result<()> {
    let date = EventDate::create_or_default(day, year);
    let input = get_input(&date, input_override_path)?;

    solution::build(&date, release_build)?;

    for &part in parts {
        let result = solution::exec(&input, part, &date, release_build)?;

        match result {
            ExecResult::Complete {
                answer,
                duration_secs,
                duration_subsec_ns,
            } => {
                if std::io::stdout().is_terminal() {
                    let duration = Duration::new(duration_secs, duration_subsec_ns);
                    println!(
                        "┌ Part {part}\n{answer}\n└ In {duration:#?} ",
                        answer = answer
                            .lines()
                            .map(|line| format!("│  \x1b[32m{line}\x1b[0m"))
                            .collect::<Vec<_>>()
                            .join("\n"),
                        duration = duration,
                    )
                } else {
                    println!("{answer}")
                }
            }
            ExecResult::Failed(error) => {
                if std::io::stderr().is_terminal() {
                    eprintln!(
                        "\x1b[31m┌ Part {part} failed\n{error}\n└\x1b[0m",
                        error = error
                            .lines()
                            .map(|line| format!("│  {line}"))
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );
                } else {
                    eprintln!("{error}")
                }
            }
        }
    }

    Ok(())
}
