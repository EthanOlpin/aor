use std::fmt::Display;

use regex::Regex;

use crate::solution::Part;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlock {
    pub content: String,
    pub emphasized: bool,
}

impl CodeBlock {
    pub fn multiline(&self) -> bool {
        self.content.contains('\n')
    }
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

fn html_unescape(input: &str) -> String {
    input
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn clean_html(input: &str) -> String {
    let re = Regex::new(r"(?s)<[^>]*>").unwrap();
    html_unescape(&re.replace_all(input, "").trim_end())
}

pub fn parse_code_blocks(problem_html: &str) -> Vec<CodeBlock> {
    let code_block_re = Regex::new(r#"<code>(?s:(.*?))</code>"#).unwrap();
    let emphasized_re = Regex::new(r"^<em>(?s:(.*?))</em>$").unwrap();
    let mut code_blocks = Vec::new();
    for cap in code_block_re.captures_iter(problem_html) {
        let content = cap.get(1).unwrap().as_str();
        if let Some(emphasized_cap) = emphasized_re.captures(&content) {
            let emphasized_content = emphasized_cap.get(1).unwrap().as_str();
            code_blocks.push(CodeBlock {
                content: clean_html(&emphasized_content),
                emphasized: true,
            });
        } else {
            code_blocks.push(CodeBlock {
                content: clean_html(content),
                emphasized: false,
            });
        }
    }
    code_blocks
}

pub fn previous_answers(problem_html: &str) -> Vec<String> {
    let answer_re = Regex::new(r#"(?s:Your puzzle answer was\s*<code>(.*?)</code>)"#).unwrap();
    let mut answers = Vec::new();
    for cap in answer_re.captures_iter(problem_html) {
        let answer = cap.get(1).unwrap().as_str().to_owned();
        answers.push(answer);
    }
    answers
}

pub fn part_two_unlocked(problem_html: &str) -> bool {
    problem_html.contains("--- Part Two ---")
}

pub fn unlocked_parts(problem_html: &str) -> Vec<Part> {
    if part_two_unlocked(problem_html) {
        vec![Part::One, Part::Two]
    } else {
        vec![Part::One]
    }
}
