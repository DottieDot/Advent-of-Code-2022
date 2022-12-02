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

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum HandShape {
  Rock,
  Paper,
  Scissors
}

impl HandShape {
  fn score(self) -> u8 {
    match self {
      HandShape::Rock => 1,
      HandShape::Paper => 2,
      HandShape::Scissors => 3
    }
  }

  fn play_against(self, opponent: HandShape) -> MatchResult {
    match (self, opponent) {
      (HandShape::Rock, HandShape::Rock) => MatchResult::Draw,
      (HandShape::Rock, HandShape::Paper) => MatchResult::Lose,
      (HandShape::Rock, HandShape::Scissors) => MatchResult::Win,
      (HandShape::Paper, HandShape::Rock) => MatchResult::Win,
      (HandShape::Paper, HandShape::Paper) => MatchResult::Draw,
      (HandShape::Paper, HandShape::Scissors) => MatchResult::Lose,
      (HandShape::Scissors, HandShape::Rock) => MatchResult::Lose,
      (HandShape::Scissors, HandShape::Paper) => MatchResult::Win,
      (HandShape::Scissors, HandShape::Scissors) => MatchResult::Draw
    }
  }
}

impl TryFrom<char> for HandShape {
  type Error = InvalidHandShapeError;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      'A' => Ok(HandShape::Rock),
      'B' => Ok(HandShape::Paper),
      'C' => Ok(HandShape::Scissors),

      'X' => Ok(HandShape::Rock),
      'Y' => Ok(HandShape::Paper),
      'Z' => Ok(HandShape::Scissors),

      char => Err(InvalidHandShapeError { value: char })
    }
  }
}

#[derive(Error, Debug)]
#[error("{value} is not a valid hand shape")]
struct InvalidHandShapeError {
  value: char
}

#[derive(Clone, Copy, Debug)]
enum Strategy {
  Lose,
  Draw,
  Win
}

impl Strategy {
  pub fn get_hand(self, opponent: HandShape) -> HandShape {
    match (self, opponent) {
      (Strategy::Draw, _) => opponent,

      (Strategy::Lose, HandShape::Rock) => HandShape::Scissors,
      (Strategy::Lose, HandShape::Paper) => HandShape::Rock,
      (Strategy::Lose, HandShape::Scissors) => HandShape::Paper,

      (Strategy::Win, HandShape::Rock) => HandShape::Paper,
      (Strategy::Win, HandShape::Paper) => HandShape::Scissors,
      (Strategy::Win, HandShape::Scissors) => HandShape::Rock
    }
  }
}

impl TryFrom<char> for Strategy {
  type Error = InvalidStrategyError;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      'X' => Ok(Strategy::Lose),
      'Y' => Ok(Strategy::Draw),
      'Z' => Ok(Strategy::Win),
      _ => Err(InvalidStrategyError { value })
    }
  }
}

#[derive(Error, Debug)]
#[error("{value} is not a valid strategy")]
struct InvalidStrategyError {
  value: char
}

#[derive(Clone, Copy, Debug)]
enum MatchResult {
  Lose,
  Draw,
  Win
}

impl MatchResult {
  pub fn score(self) -> u8 {
    match self {
      MatchResult::Lose => 0,
      MatchResult::Draw => 3,
      MatchResult::Win => 6
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct Matchup {
  opponent: HandShape,
  me:       HandShape,
  strategy: Strategy
}

impl Matchup {
  pub fn score(self) -> (u8, u8) {
    let strategy_hand = self.strategy.get_hand(self.opponent);
    (
      self.me.score() + self.me.play_against(self.opponent).score(),
      strategy_hand.score() + strategy_hand.play_against(self.opponent).score()
    )
  }
}

impl TryFrom<&str> for Matchup {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.chars().collect::<Vec<_>>()[..] {
      [a, ' ', b] => {
        Ok(Matchup {
          opponent: HandShape::try_from(a).context("invalid opponent")?,
          me:       HandShape::try_from(b).context("invalid me")?,
          strategy: Strategy::try_from(b).context("invalid strategy")?
        })
      }
      _ => {
        Err(
          InvalidMatchupError {
            value: value.to_string()
          }
          .into()
        )
      }
    }
  }
}

#[derive(Error, Debug)]
#[error("\"{value}\" is not a valid matchup")]
struct InvalidMatchupError {
  value: String
}

fn read_input_file(path: String) -> anyhow::Result<String> {
  fs::read_to_string(path).context("failed to open input file")
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let input = read_input_file(args.input)?;

  let matchups = input
    .split("\n")
    .map(Matchup::try_from)
    .collect::<Result<Vec<_>, _>>()?;

  let (total_score_hand, total_score_strat) =
    matchups
      .iter()
      .fold((0u32, 0u32), |(hand_acc, strat_acc), matchup| {
        let (hand, strat) = matchup.score();
        (hand_acc + hand as u32, strat_acc + strat as u32)
      });

  println!(
    "Total score for hand interpretation: {total_score_hand} | Total score for strategy interpretation: {total_score_strat}"
  );

  Ok(())
}
