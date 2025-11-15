use anyhow::Result;
use aor::{
    solution::{self, Part},
    subcommands,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Run the solution for a given day
    Run {
        /// The day of the solution's problem (defaults to the day of the current date in EST)
        #[arg(short, long)]
        day: Option<u8>,
        /// The year of the solution's problem (defaults to the current year)
        #[arg(short, long)]
        year: Option<u16>,
        /// The part(s) of the solution to run, defaults to all parts
        #[arg(short, long, default_values_t=[Part::One, Part::Two])]
        parts: Vec<solution::Part>,
        /// A path to a file containing an alternate input
        #[arg(short, long)]
        input: Option<PathBuf>,
        /// Build the solution in release mode
        #[arg(long)]
        release: bool,
    },
    /// Initialize a solution for a given day
    Init {
        /// The day of the problem to solve (defaults to today OR the first day of December)
        #[arg(short, long)]
        day: Option<u8>,
        /// The year of the problem to solve (defaults to the current year)
        #[arg(short, long)]
        year: Option<u16>,
        /// Begin a countdown and delay initializing the problem until midnight
        #[arg(short, long)]
        countdown: bool,
        /// Fetch input only, refetches if input is cached
        #[arg(short, long)]
        fetch_input_only: bool,
    },
    /// Submit the answer for a given day
    Submit {
        /// The day of the problem to submit a solution for (defaults to today OR the first day of December)
        #[arg(short, long)]
        day: Option<u8>,
        /// The year of the problem to submit a solution for (defaults to the current year)
        #[arg(short, long)]
        year: Option<u16>,
        /// The specific answer to submit (defaults to the output of the day's solution)
        #[arg(short, long)]
        answer: Option<String>,
    },
    /// Generate tests using the example input and output from the problem description
    #[clap(name = "testgen")]
    GenerateTests {
        /// The day of the problem to generate tests for (defaults to today or the first day of December)
        #[arg(short, long)]
        day: Option<u8>,
        /// The year of the problem to generate tests for (defaults to today or the first day of December)
        #[arg(short, long)]
        year: Option<u16>,
        /// The part(s) of the solution to generate tests for (defaults to all unlocked parts)
        #[arg(short, long)]
        parts: Vec<solution::Part>,
    },
}

#[derive(Debug, clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let command = Args::parse().command;
    match command {
        Command::Run {
            day,
            year,
            parts,
            input,
            release: release_build,
        } => subcommands::run(day, year, &parts, input, release_build),
        Command::Init {
            day,
            year,
            countdown,
            fetch_input_only,
        } => subcommands::init(day, year, countdown, fetch_input_only),
        Command::Submit { day, year, answer } => subcommands::submit(day, year, answer),
        Command::GenerateTests { day, year, parts } => {
            subcommands::generate_tests(day, year, &parts)
        }
    }
}
