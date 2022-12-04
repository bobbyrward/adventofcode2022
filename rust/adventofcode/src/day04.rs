#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use clap::Parser;
use std::str::FromStr;

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Assignment {
    start: u64,
    end: u64,
}

impl Assignment {
    #[allow(dead_code)]
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
    fn fully_contains(&self, rhs: &Self) -> bool {
        self.start <= rhs.start && self.end >= rhs.end
    }

    fn contains(&self, rhs: &Self) -> bool {
        (self.start <= rhs.start && self.end >= rhs.start)
            || (self.start <= rhs.end && self.end >= rhs.end)
    }
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or_else(|| anyhow!("Missing -"))?;

        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn parse_line(line: &str) -> Result<(Assignment, Assignment)> {
    let (elf1, elf2) = line.split_once(',').ok_or_else(|| anyhow!("Missing ,"))?;

    Ok((elf1.parse()?, elf2.parse()?))
}

fn part_one() -> Result<String> {
    let count = input(crate::Day::day04)
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|(a, b)| a.fully_contains(&b) || b.fully_contains(&a))
        .filter(|x| *x)
        .count();

    Ok(format!("{:?}", count))
}

fn part_two() -> Result<String> {
    let count = input(crate::Day::day04)
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|(a, b)| a.contains(&b) || b.contains(&a))
        .filter(|x| *x)
        .count();

    Ok(format!("{:?}", count))
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let pairs = TEST_INPUT
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<_>>>()?;

        println!("{:?}", pairs);

        assert_eq!(
            pairs
                .iter()
                .map(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
                .collect::<Vec<_>>(),
            vec![false, false, false, true, true, false]
        );

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let pairs = TEST_INPUT
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<_>>>()?;

        println!("{:?}", pairs);

        assert_eq!(
            pairs
                .iter()
                .map(|(a, b)| a.contains(b) || b.contains(a))
                .collect::<Vec<_>>(),
            vec![false, false, true, true, true, true]
        );

        Ok(())
    }
}
