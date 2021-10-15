extern crate clap;
use clap::{App, Arg};
mod solution;
use solution::Solver;
use std::io::{self, Read};

fn main() {
  let matches = App::new("Advent of Code 2017 Solutions")
    .arg(Arg::with_name("DAY").short("d").index(1))
    .arg(Arg::with_name("PART").short("p").index(2))
    .get_matches();

  let day = matches
    .value_of("DAY")
    .unwrap_or_default()
    .parse::<usize>()
    .unwrap_or(1);
  let part = matches
    .value_of("PART")
    .unwrap_or_default()
    .parse::<usize>()
    .unwrap_or(1);

  let mut buf = String::new();
  if let Err(err) = io::stdin().read_to_string(&mut buf) {
    eprintln!("stdin().read_to_string(): error {0}", err);
    return;
  };

  let solver = Solver::new();

  let result = solver.solve(day, part, &buf);
  println!("{0}", result);
}
