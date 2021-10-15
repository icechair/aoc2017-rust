use super::Solution;
use std::collections::HashMap;
use std::ops;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(i64, i64);

impl Point {
  pub fn manhatten(self) -> i64 {
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
const DIRECTIONS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];
const DIAGONALS: [Point; 4] = [Point(1, 1), Point(-1, 1), Point(-1, -1), Point(1, -1)];

pub struct Solver {
  facing: usize,
  position: Point,
  spiral: HashMap<Point, i64>,
}
impl Solver {
  pub fn new() -> Self {
    let position = Point(0, 0);
    let mut spiral = HashMap::new();
    spiral.insert(position, 1);
    Self {
      facing: 0,
      position: Point(0, 0),
      spiral,
    }
  }

  fn walk(&mut self, until: i64, with_accumulation: bool) -> Point {
    let mut n = 1;
    let mut acc;
    while n < until {
      log::info!(
        "day3::walk(): n = {:?}, pos: {:?},  left: {:?}, forward: {:?}",
        n,
        self.position,
        self.left(),
        self.facing,
      );

      self.forward(n);

      if with_accumulation {
        acc = self.adjacent();
        self.spiral.insert(self.position, acc);
        log::info!("\tadjacent: {}", acc);
      } else {
        acc = n;
      }

      if until < acc {
        return self.position;
      }

      if self.is_left_empty() {
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

  fn adjacent(&self) -> i64 {
    let mut sum = 0;
    for d in DIRECTIONS.iter().chain(DIAGONALS.iter()) {
      let point = self.position + *d;
      if let Some(p) = self.spiral.get(&point) {
        log::info!("adjacent point: {:?} has = {}", point, p);
        sum += p;
      }
    }
    return sum;
  }

  fn forward(&mut self, n: i64) {
    self.position = self.position + DIRECTIONS[self.facing];
    log::info!("forward: {}: {:?}", n, self.position);
    self.spiral.insert(self.position, n);
  }

  fn left(&self) -> Point {
    self.position + DIRECTIONS[(self.facing + 1) % DIRECTIONS.len()]
  }
  fn turn_left(&mut self) {
    self.facing = (self.facing + 1) % DIRECTIONS.len();
    log::info!("turn_left: {}", self.facing);
  }

  fn is_left_empty(&mut self) -> bool {
    self.spiral.contains_key(&self.left()) == false
  }
}

impl Solution for Solver {
  fn part1(&mut self, input: &str) -> String {
    let trimmed = input.trim();
    let mut distance = 0;
    if let Ok(until) = trimmed.parse::<i64>() {
      let p = self.walk(until, false);
      distance = p.manhatten();
    }
    return format!("{0}", distance);
  }

  fn part2(&mut self, input: &str) -> String {
    let trimmed = input.trim();
    let mut distance = 0;
    if let Ok(until) = trimmed.parse::<i64>() {
      let p = self.walk(until, true);
      if let Some(x) = self.spiral.get(&p) {
        distance = *x;
      }
    }
    return format!("{0}", distance);
  }
}
