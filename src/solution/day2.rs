use super::Solution;
pub struct Solver {}
use log::info;

fn line_difference(line: &str) -> i32 {
  let mut min = i32::MAX;
  let mut max = 0;
  for num in line.split_whitespace() {
    if let Ok(n) = num.parse::<i32>() {
      if min > n {
        min = n;
      }
      if max < n {
        max = n;
      }
    }
  }
  info!(
    "day2::line_difference(): line: {}, min: {}, max: {}",
    line, min, max
  );
  return max - min;
}

fn line_division(line: &str) -> i32 {
  let mut numbers: Vec<_> = line
    .split_whitespace()
    .map(|num| num.parse::<i32>().unwrap())
    .collect();
  numbers.sort();
  numbers.reverse();
  let mut div = 0;
  for (idx, a) in numbers.iter().enumerate() {
    for b in numbers.iter().skip(idx + 1) {
      info!(
        "day2::line_division(): a = {}, b = {}, a % b = {}, a / b = {}",
        a,
        b,
        a % b,
        a / b
      );
      if a % b == 0 {
        div += a / b;
      }
    }
  }
  info!(
    "day2::line_division(): numbers: {:?}, div = {}",
    numbers, div
  );
  return div;
}

impl Solution for Solver {
  fn part1(&mut self, input: &str) -> String {
    let trimmed = input.trim();
    let mut sum = 0;
    for line in trimmed.lines() {
      sum += line_difference(line);
    }
    return format!("{0}", sum);
  }

  fn part2(&mut self, input: &str) -> String {
    let trimmed = input.trim();
    let mut sum = 0;
    for line in trimmed.lines() {
      sum += line_division(line);
    }
    return format!("{0}", sum);
  }
}
