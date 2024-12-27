use itertools::Itertools;

use crate::core::DailyChallenge;

pub struct HistorianHysteria {
  list_one: Vec<i32>,
  list_two: Vec<i32>
}

impl HistorianHysteria {
  pub fn new(lines: Vec<String>) -> HistorianHysteria {
    let pairs: Vec<(i32, i32)> = lines.iter().map(Self::parse_line).collect();
    let list_one = pairs.iter().map(|x| x.0).sorted().collect();
    let list_two = pairs.iter().map(|x| x.1).sorted().collect();
    HistorianHysteria {
      list_one,
      list_two
    }
  }

  fn parse_line(line: &String) -> (i32, i32) {
    let vec: Vec<&str> = line.split("   ").collect();
    (vec[0].parse::<i32>().unwrap(), vec[1].parse::<i32>().unwrap())
  }
}

impl DailyChallenge for HistorianHysteria {
  fn solve_part_one(&self) -> i64 {
    let mut error: i64 = 0;
    for (ix, n) in self.list_one.iter().enumerate() {
      error += i64::from((n - self.list_two[ix]).abs())
    }
    error
  }

  fn solve_part_two(&self) -> i64 {
    let count_map = self.list_two
      .iter()
      .into_grouping_map_by(|&x| x.to_owned())
      .fold(0, |acc, _key, _value| acc + 1);

    let mut similarity: i64 = 0;
    for n in self.list_one.iter() {
      let count = match count_map.get(n) {
        Some(c) => c,
        None => &0
      };
      similarity += i64::from(n * count)
    }
    similarity
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::get_lines;

  static TEST_TEXT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

  fn setup() -> HistorianHysteria {
    let lines = get_lines(TEST_TEXT.to_string());
    HistorianHysteria::new(lines)
  }

  #[test]
  fn test_solve_part_one() {
    let sut = setup();
    let result = sut.solve_part_one();
    assert_eq!(result, 11)
  }

  #[test]
  fn test_solve_part_two() {
    let sut = setup();
    let result = sut.solve_part_two();
    assert_eq!(result, 31)
  }
}