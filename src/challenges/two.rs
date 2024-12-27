use std::cmp::Ordering;

use crate::core::DailyChallenge;

pub struct RedNosedReports {
  reports: Vec<Vec<i32>>
}

impl RedNosedReports {
  pub fn new(lines: Vec<String>) -> RedNosedReports {
    let reports: Vec<Vec<i32>> = lines.iter().map(Self::parse_line).collect();
    RedNosedReports {
      reports
    }
  }

  fn parse_line(line: &String) -> Vec<i32> {
    line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect()
  }

  fn report_is_safe(&self, report: &Vec<i32>) -> bool {
    let size = report.len();
    let ordering = report[0].cmp(&report[1]);

    if ordering == Ordering::Equal {
      return false;
    }

    for (ix, level) in report.iter().enumerate() {
      // if we reached the last element, the report is safe
      if ix + 1 == size {
        return true;
      }
      let next_level = report[ix + 1];
      // if the ordering is not matching the start, the report is unsafe
      if level.cmp(&next_level) != ordering {
        break;
      }
      // if the ordering matches but the step is too large, the report is unsafe
      if (level - next_level).abs() > 3 {
        break;
      }
    };
    false
  }
}

impl DailyChallenge for RedNosedReports {
  fn solve_part_one(&self) -> i64 {
    let mut count = 0;

    for report in self.reports.iter() {
      if self.report_is_safe(report) {
        count += 1;
      }
    }

    count
  }

  fn solve_part_two(&self) -> i64 {
    let mut count = 0;

    for report in self.reports.iter() {
      if self.report_is_safe(report) {
        count += 1;
      } else {
        let size = report.len();
        for ix in 0..size {
          let mut dampened_report = report.to_vec();
          dampened_report.remove(ix);
          if self.report_is_safe(&dampened_report) {
            count += 1;
            break;
          }
        }
      }
    }

    count
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::get_lines;

  static TEST_TEXT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

  fn setup() -> RedNosedReports {
    let lines = get_lines(TEST_TEXT.to_string());
    RedNosedReports::new(lines)
  }

  #[test]
  fn test_solve_part_one() {
    let sut = setup();
    let result = sut.solve_part_one();
    assert_eq!(result, 2)
  }

  #[test]
  fn test_solve_part_two() {
    let sut = setup();
    let result = sut.solve_part_two();
    assert_eq!(result, 4)
  }
}