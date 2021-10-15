extern crate clap;
use clap::{App, Arg};
use log;
use simple_logger as sl;
use solution::Solver;
use std::io::{self, Read};
mod solution;

fn main() -> Result<(), ()> {
  let matches = App::new("Advent of Code 2017 Solutions")
    .arg(Arg::with_name("DAY").short("d").index(1))
    .arg(Arg::with_name("PART").short("p").index(2))
    .arg(Arg::with_name("v").short("v").multiple(true))
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

  let log_level = match matches.occurrences_of("v") {
    0 => log::Level::Error,
    1 => log::Level::Warn,
    2 => log::Level::Info,
    3 => log::Level::Debug,
    _ => log::Level::Trace,
  };
  if let Err(e) = sl::init_with_level(log_level) {
    eprintln!("sl::init_with_level({}): error {}", log_level, e);
    return Err(());
  }

  let mut buf = String::new();
  if let Err(e2) = io::stdin().read_to_string(&mut buf) {
    eprintln!("sl::init_with_level({}): error {}", log_level, e2);
    return Err(());
  }

  let mut solver = Solver::new();

  let result = solver.solve(day, part, &buf);

  println!("{0}", result);
  return Ok(());
}
