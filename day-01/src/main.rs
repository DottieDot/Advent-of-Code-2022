use std::fs;

use anyhow::Context;
use clap::{Parser, ValueHint};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// The input file to use
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  input: String,

  /// The number of elves to get with the most amount of calories
  #[arg(short, long, default_value = "1")]
  n_top_elves: usize
}

fn read_input_file(path: String) -> anyhow::Result<String> {
  fs::read_to_string(path).context("failed to open input file")
}

struct Elf {
  elf:      usize,
  calories: u32
}

fn get_elves_and_their_calories(path: String) -> anyhow::Result<Vec<Elf>> {
  let result = read_input_file(path)?
    .split("\n\n")
    .map(|group| {
      group
        .split("\n")
        .map(|string| string.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().fold(0u32, u32::saturating_add))
    })
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .enumerate()
    .map(|(elf, calories)| Elf { elf, calories })
    .collect::<Vec<_>>();

  Ok(result)
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let mut elves = get_elves_and_their_calories(args.input)?;
  elves.sort_unstable_by(|a, b| b.calories.cmp(&a.calories));

  println!("Elves with the most calories:");

  let top_elves = elves.iter().take(args.n_top_elves);
  for (n, Elf { elf, calories }) in top_elves.clone().enumerate() {
    println!("{n}. Elf {elf} has {calories} calories");
  }
  let total_top_elves = top_elves.fold(0u32, |acc, Elf { calories, .. }| acc + calories);
  println!("With a total of {total_top_elves} calories");

  Ok(())
}

#[derive(Error, Debug)]
#[error("no elves in input file")]
struct NoElvesError;
