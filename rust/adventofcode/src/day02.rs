use anyhow::{anyhow, Result};
use clap::Parser;
use std::cmp::Ordering;

use crate::{input, Command};

#[derive(Debug, Parser)]
pub enum Args {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl RPS {
    fn score(&self) -> i64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn beats(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn loses_to(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn for_outcome(&self, result: RoundResult) -> RPS {
        match result {
            RoundResult::Lose => self.beats(),
            RoundResult::Draw => *self,
            RoundResult::Win => self.loses_to(),
        }
    }

    fn wins_versus(&self, rhs: RPS) -> Ordering {
        if *self == rhs {
            return Ordering::Equal;
        }

        match (self, rhs) {
            (RPS::Rock, RPS::Paper) => Ordering::Less,
            (RPS::Rock, RPS::Scissors) => Ordering::Greater,
            (RPS::Paper, RPS::Scissors) => Ordering::Less,
            (RPS::Paper, RPS::Rock) => Ordering::Greater,
            (RPS::Scissors, RPS::Rock) => Ordering::Less,
            (RPS::Scissors, RPS::Paper) => Ordering::Greater,
            _ => unreachable!(),
        }
    }

    fn score_versus(&self, rhs: RPS) -> i64 {
        let result = match self.wins_versus(rhs) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        self.score() + result
    }
}

impl TryFrom<u8> for RPS {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => Ok(RPS::Rock),
            b'B' | b'Y' => Ok(RPS::Paper),
            b'C' | b'Z' => Ok(RPS::Scissors),
            _ => Err(anyhow!("Invalid value")),
        }
    }
}

impl TryFrom<u8> for RoundResult {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(RoundResult::Lose),
            b'Y' => Ok(RoundResult::Draw),
            b'Z' => Ok(RoundResult::Win),
            _ => Err(anyhow!("Invalid value")),
        }
    }
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

fn parse_line(line: &str) -> Result<(RPS, RPS)> {
    match line.trim_end().as_bytes() {
        [lhs, b' ', rhs] => Ok((RPS::try_from(*lhs)?, RPS::try_from(*rhs)?)),
        _ => Err(anyhow!("Invalid line")),
    }
}

fn parse_result_line(line: &str) -> Result<(RPS, RPS)> {
    match line.trim_end().as_bytes() {
        [lhs, b' ', rhs] => {
            let them = RPS::try_from(*lhs)?;
            let us = them.for_outcome(RoundResult::try_from(*rhs)?);

            Ok((them, us))
        }
        _ => Err(anyhow!("Invalid line")),
    }
}

fn part_one() -> Result<String> {
    Ok(input(crate::Day::day02)
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|(them, us)| us.score_versus(*them))
        .sum::<i64>()
        .to_string())
}

fn part_two() -> Result<String> {
    Ok(input(crate::Day::day02)
        .lines()
        .map(parse_result_line)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|(them, us)| us.score_versus(*them))
        .sum::<i64>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = "A Y
B X
C Z
";
    #[traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let rounds = TEST_INPUT
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<_>>>()?;

        let round_results = rounds
            .iter()
            .map(|(them, us)| us.wins_versus(*them))
            .collect::<Vec<_>>();

        assert_eq!(
            round_results,
            vec![Ordering::Greater, Ordering::Less, Ordering::Equal]
        );

        let round_scores = rounds
            .iter()
            .map(|(them, us)| us.score_versus(*them))
            .collect::<Vec<_>>();

        assert_eq!(round_scores, vec![8, 1, 6]);

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let rounds = TEST_INPUT
            .lines()
            .map(parse_result_line)
            .collect::<Result<Vec<_>>>()?;

        let round_results = rounds
            .iter()
            .map(|(them, us)| us.wins_versus(*them))
            .collect::<Vec<_>>();

        assert_eq!(
            round_results,
            vec![Ordering::Equal, Ordering::Less, Ordering::Greater]
        );

        let round_scores = rounds
            .iter()
            .map(|(them, us)| us.score_versus(*them))
            .collect::<Vec<_>>();

        assert_eq!(round_scores, vec![4, 1, 7]);

        Ok(())
    }
}
