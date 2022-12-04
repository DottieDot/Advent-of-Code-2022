use std::{fmt::Debug, fs};

use anyhow::Context;
use clap::{Parser, ValueHint};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// The input file to use
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  input: String
}

struct CleanUpRange {
  start: u32,
  end:   u32
}

impl CleanUpRange {
  pub fn contains_range(&self, other: &CleanUpRange) -> bool {
    other.start >= self.start && other.end <= self.end
  }

  pub fn overlaps_range(&self, other: &CleanUpRange) -> bool {
    let range = self.start..=self.end;

    range.contains(&other.start) || range.contains(&other.end)
  }
}

fn read_input_file(path: String) -> anyhow::Result<String> {
  fs::read_to_string(path).context("failed to open input file")
}

fn range_from_str(range: &str) -> anyhow::Result<CleanUpRange> {
  let mapped = range
    .split("-")
    .map(|nr| nr.parse::<u32>())
    .collect::<Result<Vec<_>, _>>()?;

  if mapped.len() == 2 {
    Ok(CleanUpRange {
      start: mapped[0],
      end:   mapped[1]
    })
  } else {
    Err(InvalidRangeError {
      value: range.to_owned()
    })?
  }
}

#[derive(Error, Debug)]
#[error("{value} is not a valid range string")]
struct InvalidRangeError {
  value: String
}

fn ranges_from_pair_string(pair: &str) -> anyhow::Result<(CleanUpRange, CleanUpRange)> {
  let mut ranges = pair
    .split(",")
    .map(range_from_str)
    .collect::<Result<Vec<_>, _>>()?;

  if ranges.len() == 2 {
    Ok((ranges.swap_remove(0), ranges.swap_remove(0)))
  } else {
    Err(InvalidPairError {
      value: pair.to_owned()
    })?
  }
}

#[derive(Error, Debug)]
#[error("{value} is not a valid pair string")]
struct InvalidPairError {
  value: String
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let input = read_input_file(args.input)?;

  let pairs = input
    .split("\n")
    .map(ranges_from_pair_string)
    .collect::<Result<Vec<_>, _>>()?;

  let super_sets = pairs.iter().fold(0, |acc, (a, b)| {
    if a.contains_range(b) || b.contains_range(a) {
      acc + 1
    } else {
      acc
    }
  });

  println!("number of super sets: {super_sets}");

  let overlaps = pairs.iter().fold(0, |acc, (a, b)| {
    if a.overlaps_range(b) || b.overlaps_range(a) {
      acc + 1
    } else {
      acc
    }
  });

  println!("number of overlaps: {overlaps}");

  Ok(())
}
