use super::Solution;
pub struct Solver {}

impl Solution for Solver {
  fn part1(&self, input: &str) -> String {
    let trimmed = input.trim();
    let mut prev = trimmed.chars().last().unwrap_or_default();
    let mut same = 0;
    for c in trimmed.chars() {
      if prev == c {
        same += u32::from(prev) - u32::from('0');
      }
      prev = c;
    }
    return format!("{0}", same);
  }

  fn part2(&self, input: &str) -> String {
    let trimmed = input.trim();
    let chars: Vec<_> = trimmed.chars().collect();
    let halfway = chars.len() / 2;
    let mut same = 0;
    for (idx, c) in chars.iter().enumerate() {
      let lookup = (idx + halfway) % chars.len();
      println!(
        "day1.part2(): halfway: {}, lookup: {}, idx: {}, c: {}, lc: {}",
        halfway, lookup, idx, c, chars[lookup]
      );
      if c == &chars[lookup] {
        same += u32::from(*c) - u32::from('0');
      }
    }
    return format!("{0}", same);
  }
}
