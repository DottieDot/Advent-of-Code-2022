#![feature(int_log)]

use std::fs;

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

fn get_number_for_char(char: char) -> u8 {
  if char.is_ascii_lowercase() {
    (char as u8 - 'a' as u8) + 1
  } else if char.is_ascii_uppercase() {
    (char as u8 - 'A' as u8) + 27
  } else {
    panic!("{char}")
  }
}

fn get_char_from_number(nr: u32) -> char {
  match nr {
    1..=26 => char::from_u32(('a' as u32 + nr) - 1).unwrap(),
    27..=52 => char::from_u32(('A' as u32 + nr) - 26 - 1).unwrap(),
    _ => panic!()
  }
}

struct Compartment<'a>(&'a str);

impl<'a> Compartment<'a> {
  pub fn get_bit_field(&self) -> u64 {
    let mut result = 0u64;
    for char in self.0.chars() {
      result |= 1 << (get_number_for_char(char) - 1)
    }
    result
  }
}

struct Rucksack<'a> {
  comp_a: Compartment<'a>,
  comp_b: Compartment<'a>
}

impl<'a> Rucksack<'a> {
  pub fn get_bit_field(&self) -> u64 {
    self.comp_a.get_bit_field() | self.comp_b.get_bit_field()
  }

  pub fn get_item_in_both_compartments(&self) -> Option<char> {
    let comp_a_field = self.comp_a.get_bit_field();
    let comp_b_field = self.comp_b.get_bit_field();

    let duplicates = comp_a_field & comp_b_field;

    duplicates
      .checked_ilog2()
      .map(|nr| get_char_from_number(nr + 1))
  }
}

struct Group<'a> {
  group: &'a [Rucksack<'a>]
}

impl<'a> Group<'a> {
  pub fn get_common_item(&self) -> Option<char> {
    let mut field = u64::MAX;
    for rucksack in self.group {
      field &= rucksack.get_bit_field();
    }

    field.checked_ilog2().map(|nr| get_char_from_number(nr + 1))
  }
}

fn get_rucksacks<'a>(input: &'a str) -> Vec<Rucksack<'a>> {
  input
    .split('\n')
    .map(|line| {
      let half = line.len() / 2;
      Rucksack {
        comp_a: Compartment(&line[..half]),
        comp_b: Compartment(&line[half..])
      }
    })
    .collect()
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let file = read_input_file(args.input)?;

  let rucksacks = get_rucksacks(&file);

  let result_part1 = rucksacks.iter().fold(0u32, |acc, r| {
    acc + get_number_for_char(r.get_item_in_both_compartments().unwrap()) as u32
  });

  println!("total value of wrongly sorted items: {result_part1}");

  let mut sum = 0u32;
  for g in (0..(rucksacks.len() / 3)).map(|n| n * 3) {
    let items = &rucksacks[g..g + 3];

    let group = Group { group: items };

    sum += get_number_for_char(group.get_common_item().unwrap()) as u32
  }

  println!("total value of group items: {sum}");

  Ok(())
}
