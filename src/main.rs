use dotenv::dotenv;
use std::io::stdin;
use core::DailyChallenge;
use challenges::*;

mod challenges;
mod core;

fn main() {
  dotenv().ok();

  println!("Which day would you like to run?");
  let mut day_str = String::new();
  stdin().read_line(&mut day_str).expect("Can't read input!");

  let day = day_str.trim().parse::<u16>().unwrap();
  let challenge = get_challenge(day);
  
  println!("--- Day {} ---", day);
  println!("Part 1: {:?}", challenge.solve_part_one());
  println!("Part 2: {:?}", challenge.solve_part_two());
}

fn get_challenge(day: u16) -> Box<dyn DailyChallenge> {
  if day > 25 {
    panic!("Day cannot be over 25!");
  }
  let input = core::get_input(&day);
  match day {
    1 => Box::new(one::HistorianHysteria::new(input)),
    2 => Box::new(two::RedNosedReports::new(input)),
    _ => panic!("Day {} not yet implemented!", day)
  }
}