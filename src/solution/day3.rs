use super::Solution;
use std::collections::HashSet;
use std::ops;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(i32, i32);

impl Point {
  pub fn manhatten(self) -> i32 {
    self.0.abs() + self.1.abs()
  }
}

impl ops::Add for Point {
  type Output = Self;
  fn add(self, _rhs: Self) -> Self {
    Self(self.0 + _rhs.0, self.1 + _rhs.1)
  }
}
//RIGHT, UP, LEFT, DOWN
const directions: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];

pub struct Solver {
  facing: usize,
  position: Point,
  spiral: HashSet<Point>,
}
impl Solver {
  pub fn new() -> Self {
    let position = Point(0, 0);
    let mut spiral = HashSet::new();
    spiral.insert(position);
    Self {
      facing: 0,
      position: Point(0, 0),
      spiral,
    }
  }

  fn walk(&mut self, until: i32) -> Point {
    let mut n = 1;
    while n < until {
      log::info!(
        "day3::walk(): n = {:?}, pos: {:?},  left: {:?}, forward: {:?}",
        n,
        self.position,
        self.left(),
        self.facing,
      );
      self.forward();
      log::info!("\tforward");
      if self.is_left_empty() {
        log::info!("\tturn left");
        self.turn_left();
      }
      n += 1;
    }
    log::info!(
      "day3::walk_done(): n: {:?}, pos: {:?}\n\tspiral:{:?}",
      n,
      self.position,
      self.spiral
    );
    return self.position;
  }

  fn forward(&mut self) {
    self.position = self.position + directions[self.facing];
    self.spiral.insert(self.position);
  }

  fn left(&self) -> Point {
    self.position + directions[(self.facing + 1) % directions.len()]
  }
  fn turn_left(&mut self) {
    self.facing = (self.facing + 1) % directions.len()
  }

  fn is_left_empty(&mut self) -> bool {
    self.spiral.contains(&self.left()) == false
  }
}

impl Solution for Solver {
  fn part1(&mut self, input: &str) -> String {
    let trimmed = input.trim();
    let mut distance = 0;
    if let Ok(until) = trimmed.parse::<i32>() {
      let p = self.walk(until);
      distance = p.manhatten();
    }
    return format!("{0}", distance);
  }

  fn part2(&mut self, input: &str) -> String {
    let trimmed = input.trim();
    let mut sum = 0;
    return format!("{0}", sum);
  }
}
