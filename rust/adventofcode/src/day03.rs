use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::HashSet;

use crate::{input, Command};

#[derive(Debug, Parser)]
pub enum Args {
    Part1,
    Part2,
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TypePriority(u8);

impl TryFrom<char> for TypePriority {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='z' => Ok(Self(c as u8 - b'a' + 1)),
            'A'..='Z' => Ok(Self(c as u8 - b'A' + 27)),
            _ => Err(anyhow!("Invalid item type value")),
        }
    }
}

impl From<TypePriority> for u8 {
    fn from(tp: TypePriority) -> Self {
        tp.0
    }
}

impl From<TypePriority> for u16 {
    fn from(tp: TypePriority) -> Self {
        tp.0 as Self
    }
}

impl From<TypePriority> for u32 {
    fn from(tp: TypePriority) -> Self {
        tp.0 as Self
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Rucksack {
    first_half: HashSet<char>,
    second_half: HashSet<char>,
}

impl Rucksack {
    fn new<A, B>(first_half: A, second_half: B) -> Self
    where
        A: IntoIterator<Item = char>,
        B: IntoIterator<Item = char>,
    {
        Self {
            first_half: first_half.into_iter().collect(),
            second_half: second_half.into_iter().collect(),
        }
    }

    fn union(&self) -> HashSet<char> {
        self.first_half
            .union(&self.second_half)
            .into_iter()
            .copied()
            .collect()
    }

    fn common_type(&self) -> Result<char> {
        let intersection = self
            .first_half
            .intersection(&self.second_half)
            .collect::<Vec<_>>();

        if intersection.len() != 1 {
            return Err(anyhow!("Unexpected number of common item types"));
        }

        Ok(*intersection[0])
    }
}

fn parse_line(line: &str) -> Rucksack {
    let line = line.trim_end();
    let length = line.len();

    let first_half = &line[..length / 2];
    let second_half = &line[length / 2..length];

    Rucksack::new(first_half.chars(), second_half.chars())
}

fn part_one() -> Result<String> {
    let sum: u32 = input(crate::Day::day03)
        .lines()
        .map(parse_line)
        .map(|rs| rs.common_type())
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(TypePriority::try_from)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(u32::from)
        .sum::<u32>();

    Ok(format!("{}", sum))
}

fn find_badges(input: &str) -> Result<Vec<char>> {
    input
        .lines()
        .map(parse_line)
        .chunks(3)
        .into_iter()
        .map(|c| {
            c.map(|rs| rs.union())
                .fold(None, |acc: Option<HashSet<char>>, x| {
                    if let Some(hs) = acc {
                        Some(hs.intersection(&x).into_iter().copied().collect())
                    } else {
                        Some(x)
                    }
                })
                .unwrap()
                .into_iter()
                .exactly_one()
                .map_err(anyhow::Error::from)
        })
        .collect::<Result<Vec<_>>>()
}

fn part_two() -> Result<String> {
    Ok(find_badges(input(crate::Day::day03))?
        .into_iter()
        .map(TypePriority::try_from)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(u32::from)
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let parsed = TEST_INPUT.lines().map(parse_line).collect::<Vec<_>>();
        let common = parsed
            .iter()
            .map(|rs| rs.common_type())
            .collect::<Result<Vec<_>>>()?;
        let common_priorities = common
            .iter()
            .map(|c| TypePriority::try_from(*c))
            .collect::<Result<Vec<_>>>()?;

        println!("Parsed {:?}", parsed);
        println!("Common {:?}", common);
        println!("Priorities {:?}", common_priorities);

        assert_eq!(common, vec!['p', 'L', 'P', 'v', 't', 's']);
        assert_eq!(
            common_priorities,
            vec![
                TypePriority(16),
                TypePriority(38),
                TypePriority(42),
                TypePriority(22),
                TypePriority(20),
                TypePriority(19)
            ]
        );

        assert_eq!(
            157u32,
            common_priorities.into_iter().map(|tp| tp.0 as u32).sum()
        );

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let badges = find_badges(TEST_INPUT)?;

        println!("{:?}", badges);
        assert_eq!(badges, vec!['r', 'Z']);
        assert_eq!(
            badges
                .into_iter()
                .map(TypePriority::try_from)
                .collect::<Result<Vec<_>>>()?,
            vec![TypePriority(18), TypePriority(52)]
        );
        Ok(())
    }
}
