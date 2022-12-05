use std::{collections::VecDeque, fs};

use anyhow::Context;
use clap::{Parser, ValueHint};
use regex::Regex;

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

fn split_input(input: &str) -> (&str, &str) {
  let pos = input.find("\n\n").unwrap();

  (&input[0..pos], &input[pos + 2..])
}

fn get_stacks(input: &str) -> Vec<VecDeque<char>> {
  let diagram = input
    .split("\n")
    .take_while(|line| !line.contains("1"))
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let width = diagram[0].len();
  let n_stacks = width / 4;

  (0..=n_stacks)
    .map(|column| {
      diagram
        .iter()
        .map(move |line| line[column * 4 + 1])
        .rev()
        .take_while(|char| *char != ' ')
        .collect::<VecDeque<_>>()
    })
    .collect::<Vec<_>>()
}

struct Move {
  count: usize,
  from:  usize,
  to:    usize
}

impl Move {
  pub fn apply(&self, to: &mut Vec<VecDeque<char>>) {
    for _ in 0..self.count {
      let item = to[self.from].pop_back().unwrap();
      to[self.to].push_back(item);
    }
  }

  pub fn apply_9001(&self, to: &mut Vec<VecDeque<char>>) {
    let len = to[self.from].len();
    let mut drain = to[self.from]
      .drain(len - self.count..)
      .collect::<VecDeque<_>>();
    to[self.to].append(&mut drain);
  }
}

fn parse_moves(input: &str) -> Vec<Move> {
  lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
  }

  input
    .split("\n")
    .map(|line| {
      let captures = RE.captures(line).expect("invalid puzzle input");
      Move {
        count: captures[1].parse().unwrap(),
        from:  captures[2].parse::<usize>().unwrap() - 1,
        to:    captures[3].parse::<usize>().unwrap() - 1
      }
    })
    .collect()
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();
  let input = read_input_file(args.input)?;

  let (diagram, moves_input) = split_input(&input);

  let stacks = get_stacks(diagram);
  let moves = parse_moves(moves_input);

  println!("{stacks:#?}");

  let mut result_9000 = stacks.clone();
  for mve in &moves {
    mve.apply(&mut result_9000);
  }

  let top_containers = result_9000
    .iter()
    .map(|stack| stack.back().unwrap())
    .collect::<String>();

  println!("Top containers 9000: {top_containers}");

  let mut result_9001 = stacks.clone();
  for mve in &moves {
    mve.apply_9001(&mut result_9001);
  }

  let top_containers = result_9001
    .iter()
    .map(|stack| stack.back().unwrap())
    .collect::<String>();

  println!("Top containers 9001: {top_containers}");

  Ok(())
}
