use std::{fs, io::Write, path::Path, time::Duration};

use crate::{
    aoc_client,
    event_date::EventDate,
    solution::{self, solution_path},
};
use anyhow::anyhow;
use chrono::{Local, TimeZone};
use chrono_tz;

fn block_and_countdown(until_date: &EventDate) -> anyhow::Result<()> {
    let now = Local::now().with_timezone(&chrono_tz::EST);
    let start = chrono_tz::EST
        .with_ymd_and_hms(until_date.year.into(), 12, until_date.day.into(), 0, 0, 0)
        .earliest()
        .ok_or(anyhow!("Not a valid date"))?;

    if start < now {
        return Err(anyhow!("Can't countdown to event in the past"));
    }

    let seconds = (start - now).num_seconds() + 1;

    for remaining_seconds in (1..=seconds).rev() {
        print!("\x1b[2K\rInitializing in: {remaining_seconds}s");
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn maybe_open_in_editor(path: &Path) -> anyhow::Result<()> {
    let editor = std::env::var("VISUAL");

    if let Ok(editor) = editor {
        std::process::Command::new(editor).arg(path).status()?;
    };

    Ok(())
}

pub fn init(
    day: Option<u8>,
    year: Option<u16>,
    countdown: bool,
    fetch_input_only: bool,
) -> anyhow::Result<()> {
    let date = EventDate::create_or_next(day, year);
    if countdown {
        block_and_countdown(&date)?;
    }
    if !fetch_input_only {
        let solution_path = solution_path(&date);
        if solution_path.exists() {
            return Err(anyhow!(
                "Solution already initialized at {}",
                solution_path.to_string_lossy()
            ));
        }
        let mut template = fs::read_to_string("src/template.rs")?;
        template = template
            .replace("{{dd}}", &format!("{:02}", date.day))
            .replace("{{d}}", &date.day.to_string())
            .replace("{{yyyy}}", &date.year.to_string());
        fs::write(&solution_path, template)?;

        // Append to Cargo.toml
        let bin_entry = &format!(
            "\n[[bin]]\nname = \"{}\"\npath = \"{}\"\n",
            solution::bin_name(&date),
            solution_path.to_string_lossy()
        );

        let mut cargo_toml_file = fs::OpenOptions::new().append(true).open("Cargo.toml")?;
        cargo_toml_file.write_all(bin_entry.as_bytes())?;

        println!(
            "Initialized solution from template at: {}",
            solution_path.canonicalize()?.to_string_lossy()
        );
        maybe_open_in_editor(&solution_path)?;
    }

    aoc_client::get_input(&date).map_err(|err| anyhow!("Failed to pre-fetch input: {err}"))?;
    Ok(())
}
