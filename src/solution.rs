mod day1;
mod day2;
mod day3;

trait Solution {
  fn part1(&mut self, input: &str) -> String;
  fn part2(&mut self, input: &str) -> String;
}
impl Solution for Box<dyn Solution> {
  fn part1(&mut self, input: &str) -> String {
    self.as_mut().part1(input)
  }
  fn part2(&mut self, input: &str) -> String {
    self.as_mut().part2(input)
  }
}

pub struct Solver {
  solutions: Vec<Box<dyn Solution>>,
}

impl Solver {
  pub fn new() -> Self {
    let mut solutions: Vec<Box<dyn Solution>> = Vec::new();
    solutions.push(Box::new(day1::Solver {}));
    solutions.push(Box::new(day2::Solver {}));
    solutions.push(Box::new(day3::Solver::new()));
    return Self { solutions };
  }

  pub fn solve(&mut self, day: usize, part: usize, input: &str) -> String {
    let s = &mut self.solutions[day - 1];
    match part {
      2 => s.part2(input),
      _ => s.part1(input),
    }
  }
}
