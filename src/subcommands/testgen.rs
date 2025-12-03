use std::{fmt::Display, fs, path::PathBuf};

use itertools::Itertools as _;

use crate::{
    aoc_client,
    event_date::EventDate,
    problem::{self, CodeBlock},
    solution::{Part, solution_path},
};

fn indent(text: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<String>>()
        .join("\n")
}

fn example_input_heuristic(code_block: &CodeBlock) -> u32 {
    let CodeBlock {
        content,
        emphasized: _,
    } = code_block;

    let mut score = 0;

    if content.contains("\n") {
        score += 100;
    }

    score += content.len().min(100) as u32;
    score
}

fn example_solution_heuristic(code_block: &CodeBlock) -> u32 {
    let CodeBlock {
        content,
        emphasized,
    } = code_block;

    let mut score: u32 = 0;
    if *emphasized {
        score += 100;
    }

    if content.len() < 100 {
        if content.chars().all(|c| c.is_ascii_digit()) {
            score = score.saturating_add(content.parse::<u32>().unwrap_or(u32::MAX));
        } else {
            score += content.len() as u32;
        }
    }

    score
}

fn prompt_select<'a, T: Display>(
    prompt: &str,
    options: &'a Vec<T>,
    page_size: usize,
) -> anyhow::Result<&'a T> {
    let pages = options.chunks(page_size).collect::<Vec<_>>();
    fn display_page<T: Display>(page: &[T], offset: usize) {
        for (i, option) in page.iter().enumerate() {
            println!("{}:\n{}", i + 1 + offset, indent(&option.to_string(), 4));
        }
    }

    for (i, page) in pages.iter().enumerate() {
        display_page(page, i * page_size);
        if i + 1 < pages.len() {
            print!("\n{prompt} (Press Enter to see more): ");
        } else {
            print!("\n{prompt}");
        }
        std::io::Write::flush(&mut std::io::stdout())?;

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().is_empty() {
                break;
            }

            if let Some(option) = input.trim().parse::<usize>().ok() {
                if option >= 1 && option <= options.len() {
                    return Ok(&options[option - 1]);
                } else {
                    Err(anyhow::anyhow!("Invalid option number"))?;
                };
            }
        }
    }
    unreachable!();
}

pub fn generate_tests(day: Option<u8>, year: Option<u16>, parts: &[Part]) -> anyhow::Result<()> {
    let date = EventDate::create_or_default(day, year);
    let problem = aoc_client::get_problem(&date)?;
    let code_blocks = problem::parse_code_blocks(&problem);
    let page_size = 5;
    let previous_answers = problem::previous_answers(&problem);
    let unlocked_parts = problem::unlocked_parts(&problem);
    let parts = if parts.is_empty() {
        &unlocked_parts
    } else {
        parts
    };

    let example_inputs = code_blocks
        .clone()
        .into_iter()
        .filter(|cb| !previous_answers.contains(&cb.content))
        .sorted_by_key(example_input_heuristic)
        .rev()
        .dedup()
        .collect::<Vec<_>>();

    let example_solutions = code_blocks
        .into_iter()
        .filter(|cb| !previous_answers.contains(&cb.content))
        .sorted_by_key(example_solution_heuristic)
        .rev()
        .dedup()
        .collect::<Vec<_>>();

    let solution_file_path = solution_path(&date);

    for part in parts {
        let input_selection = prompt_select(
            &format!("Select an input to use for Part {part}"),
            &example_inputs,
            page_size,
        )?;

        let solution_selection = prompt_select(
            &format!("Select an example solution to use for Part {part}"),
            &example_solutions,
            page_size,
        )?;

        append_test_case(
            &solution_file_path,
            part,
            &input_selection.content,
            &solution_selection.content,
        )?;

        println!(
            "Generated test for Part {part}:\nInput:\n{input}\nExpected Output:\n{expected_output}\n",
            part = part,
            input = input_selection.content,
            expected_output = solution_selection.content
        );
    }

    println!("Tests appended to {solution_file_path:?}");
    Ok(())
}

fn ensure_test_module(file_path: &PathBuf) -> anyhow::Result<()> {
    let content = fs::read_to_string(file_path)?;

    if content.contains("#[cfg(test)]") {
        return Ok(());
    }

    let test_module = r#"

#[cfg(test)]
mod tests {
    use super::*;
}
"#;

    fs::write(file_path, content + test_module)?;
    Ok(())
}

fn append_test_case(
    file_path: &PathBuf,
    part: &Part,
    input: &str,
    expected_output: &str,
) -> anyhow::Result<()> {
    ensure_test_module(file_path)?;

    let content = fs::read_to_string(file_path)?;
    let test_case = write_test_case(part, input, expected_output);

    let test_module_end = content
        .rfind('}')
        .ok_or_else(|| anyhow::anyhow!("Could not find closing brace of test module"))?;

    let new_content = format!(
        "{}\n{}\n{}",
        &content[..test_module_end],
        test_case,
        &content[test_module_end..]
    );

    fs::write(file_path, new_content)?;
    Ok(())
}

fn write_test_case(part: &Part, input: &str, expected_output: &str) -> String {
    format!(
        r##"
    #[test]
    fn test_part{part_num}() {{
        let input = r#"{input}"#.to_string();
        let expected_output = r#"{expected_output}"#.to_string();
        let result = part{part_num}(input).unwrap();
        assert_eq!(result, expected_output);
    }}"##,
        part_num = part.to_int(),
        input = input,
        expected_output = expected_output
    )
}
