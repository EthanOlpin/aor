use crate::event_date::EventDate;
use anyhow::{Result, anyhow};
use regex::Regex;
use reqwest::{header, redirect};
use std::env;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::str::FromStr;

const AOC_URL: &str = "https://adventofcode.com";
const AOC_AUTH_TOKEN: &str = "AOC_AUTH_TOKEN";
const CACHE_DIR: &str = env!("AOC_CACHE_DIR");

#[derive(Clone, Copy)]
enum Extension {
    Txt,
    Html,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Extension::Txt => "txt",
            Extension::Html => "html",
        };
        f.write_str(s)
    }
}

struct CacheOptions<'a> {
    resource: &'a String,
    extension: Extension,
    bust_cache: bool,
}

fn get_client() -> reqwest::blocking::Client {
    let auth_token =
        env::var(AOC_AUTH_TOKEN).expect(&format!("Expected {} to be set", AOC_AUTH_TOKEN));
    let mut headers = header::HeaderMap::with_capacity(1);
    let cookie: header::HeaderValue = format!("session={auth_token}").parse().unwrap();
    headers.try_insert(header::COOKIE, cookie).unwrap();
    reqwest::blocking::ClientBuilder::new()
        .user_agent("aot (+https://github.com/EthanOlpin)")
        .default_headers(headers)
        .redirect(redirect::Policy::none())
        .build()
        .unwrap()
}

fn get(path: &str) -> Result<String> {
    let url = reqwest::Url::from_str(AOC_URL)?.join(path)?;
    let response = get_client().get(url).send()?;
    let result = response.error_for_status()?;
    let text = result.text()?.trim().to_string();
    Ok(text)
}

fn post(path: &str, body: &str) -> Result<String> {
    let url = reqwest::Url::from_str(AOC_URL)?.join(path)?;
    let response = get_client()
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body.to_string())
        .send()?;
    if response.status() == 303 {
        let location = response
            .headers()
            .get("location")
            .and_then(|location| location.to_str().ok())
            .unwrap_or("unknown");

        return Err(anyhow!("POST {path} would redirect to {location}"));
    }
    let result = response.error_for_status()?;
    let text = result.text()?;
    Ok(text)
}

fn cached<F: FnOnce() -> Result<String>>(options: CacheOptions, f: F) -> Result<String> {
    let CacheOptions {
        resource,
        extension,
        bust_cache,
    } = options;
    let cache_path = Path::new(CACHE_DIR).join(format!("{resource}.{extension}"));
    if !bust_cache && cache_path.exists() {
        let cached_contents = fs::read_to_string(cache_path)?;
        return Ok(cached_contents);
    }

    let result = f()?;

    if let Some(cache_dir) = cache_path.parent() {
        fs::create_dir_all(cache_dir)?;
        fs::write(cache_path, &result)?;
    }
    Ok(result)
}

fn _get_problem(date: &EventDate, bust_cache: bool) -> Result<String> {
    let resource = format!("{}/day/{}", date.year, date.day);
    let options = CacheOptions {
        resource: &resource,
        extension: Extension::Html,
        bust_cache,
    };
    cached(options, || get(&resource))
}

pub fn get_problem(date: &EventDate) -> Result<String> {
    _get_problem(&date, false)
}
pub fn get_refreshed_problem(date: &EventDate) -> Result<String> {
    _get_problem(date, true)
}

pub fn get_input(date: &EventDate) -> Result<String> {
    let resource = format!("{}/day/{}/input", date.year, date.day);
    let options = CacheOptions {
        resource: &resource,
        extension: Extension::Txt,
        bust_cache: false,
    };
    cached(options, || get(&resource))
}

pub fn post_answer(date: &EventDate, part: u8, answer: String) -> Result<()> {
    let body = format!("level={part}&answer={answer}");
    let resource = format!("{}/day/{}/answer", date.year, date.day);
    let response = post(&resource, &body)?;
    if response.contains("That's the right answer") {
        return Ok(());
    } else {
        let message = Regex::new(r"<main>(?s:(?P<main>.*))</main>")
            .unwrap()
            .captures(&response)
            .map(|captures| captures.name("main").unwrap().as_str().to_string())
            .unwrap_or_else(|| format!("Full response: {response}"));
        return Err(anyhow!("Submission rejected: {message}"));
    }
}

pub fn refresh_problem(date: &EventDate) -> Result<()> {
    _get_problem(date, true)?;
    Ok(())
}
