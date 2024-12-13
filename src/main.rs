use core::DailyChallenge;

use dotenv::dotenv;
use one::HistorianHysteria;

mod one;
mod core;

fn main() {
  dotenv().ok();

  let input = core::get_input(&1);
  let challenge = HistorianHysteria::new(input);
  println!("Part 1: {:?}", challenge.solve_part_one());
  println!("Part 2: {:?}", challenge.solve_part_two());
}