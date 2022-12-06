use std::{collections::HashSet, fs};

use anyhow::Context;
use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// The input file to use
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  input: String
}

fn read_input_file(path: String) -> anyhow::Result<String> {
  fs::read_to_string(path).context("failed to open input file")
}

fn find_marker(string: &str, n_distinct_chars: usize) -> Option<usize> {
  let chars = string.chars().collect::<Vec<_>>();
  for index in 0..(chars.len() - n_distinct_chars) {
    let set: HashSet<char> = chars
      .iter()
      .skip(index)
      .take(n_distinct_chars)
      .map(|char| *char)
      .collect();
    if set.len() == n_distinct_chars {
      return Some(index + n_distinct_chars);
    }
  }

  None
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let input = read_input_file(args.input)?;

  println!(
    "Column of laster character in start marker with size 4: {:?}",
    find_marker(&input, 4)
  );
  println!(
    "Column of laster character in start marker with size 14: {:?}",
    find_marker(&input, 14)
  );

  Ok(())
}
