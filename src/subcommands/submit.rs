use crate::{aoc_client, event_date::EventDate, problem, solution};
use anyhow::anyhow;

pub fn submit(day: Option<u8>, year: Option<u16>, answer: Option<String>) -> anyhow::Result<()> {
    let date = EventDate::create_or_default(day, year);
    let problem_html = aoc_client::get_problem(&date)?;
    let part = if problem::part_two_unlocked(&problem_html) {
        solution::Part::Two
    } else {
        solution::Part::One
    };
    let answer: String = match answer {
        Some(answer) => Ok::<String, anyhow::Error>(answer),
        None => {
            let input = aoc_client::get_input(&date).unwrap();
            solution::build(&date, true)?;
            let result = solution::exec(&input, part, &date, true)?;
            match result {
                solution::ExecResult::Complete { answer, .. } => Ok(answer),
                solution::ExecResult::Failed(error) => {
                    Err(anyhow!("Solution to Part {part} failed:\n{error}"))
                }
            }
        }
    }?;
    aoc_client::post_answer(&date, part.to_int(), answer)?;
    println!(
        "Submitted answer for Day {} Part {}",
        date.day,
        part.to_int(),
    );
    aoc_client::refresh_problem(&date)?;
    Ok(())
}
