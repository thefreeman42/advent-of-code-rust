use std::env;

use reqwest::{blocking, header};

pub trait DailyChallenge {
  fn solve_part_one(&self) -> i64;
  fn solve_part_two(&self) -> i64;
}

fn get_cookie_header() -> String {
  let session_key = env::var("AOC_SESSION_KEY").unwrap();
  format!("session={}", session_key)
}

pub fn get_input(day: &u16) -> Vec<String> {
  let client = blocking::Client::new();
  let url = format!("https://adventofcode.com/2024/day/{}/input", day);
  let text = client.get(url).header(header::COOKIE, get_cookie_header()).send().unwrap().text().unwrap();
  let lines = text.lines().map(|l| l.to_owned()).collect();
  lines
}