use anyhow::Result;
use clap::Parser;

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

fn parse_inventories(input: &str) -> Result<Vec<i64>> {
    input
        .split("\n\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| {
            x.trim_end()
                .split('\n')
                .map(|s| s.trim().parse::<i64>().map_err(|e| e.into()))
                .collect::<Result<Vec<_>>>()
                .map(|v| v.into_iter().sum::<i64>())
        })
        .collect::<Result<Vec<_>>>()
}

fn part_one() -> Result<String> {
    let inventories = parse_inventories(input(crate::Day::day01))?;
    Ok(format!("{:?}", inventories.into_iter().max()))
}

fn find_top_3(inventories: &[i64]) -> (i64, i64, i64) {
    inventories.iter().fold((0, 0, 0), |max, x| {
        match ((*x > max.0), (*x > max.1), (*x > max.2)) {
            (true, _, _) => (*x, max.0, max.1),
            (false, true, _) => (max.0, *x, max.1),
            (false, false, true) => (max.0, max.1, *x),
            _ => max,
        }
    })
}

fn part_two() -> Result<String> {
    let inventories = parse_inventories(input(crate::Day::day01))?;
    let top3 = find_top_3(&inventories);

    Ok(format!("{:?}", top3.0 + top3.1 + top3.2))
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
    #[traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let inventories = parse_inventories(TEST_INPUT)?;
        assert_eq!(inventories, vec![6000, 4000, 11000, 24000, 10000]);

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let inventories = parse_inventories(TEST_INPUT)?;
        assert_eq!(find_top_3(&inventories), (24000, 11000, 10000));
        Ok(())
    }
}
