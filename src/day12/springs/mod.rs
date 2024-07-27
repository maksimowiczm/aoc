use crate::day12::springs::sprint_row::SpringRow;
use std::str::FromStr;

mod spring;
mod sprint_row;

pub trait Arrangements {
    fn arrangements_bogo_count(&self) -> u64;
}

pub trait FromFolds {
    fn from_folds(input: &str, folds: u8) -> Result<Self, ()>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Springs {
    rows: Vec<SpringRow>,
}

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .split("\n")
            .filter(|line| line.trim().len() > 0)
            .collect::<Vec<_>>();

        let rows = lines
            .iter()
            .copied()
            .flat_map(SpringRow::from_str)
            .collect::<Vec<_>>();

        Ok(Springs { rows })
    }
}

impl Arrangements for Springs {
    fn arrangements_bogo_count(&self) -> u64 {
        self.rows
            .iter()
            .fold(0, |acc, row| acc + row.arrangements_bogo_count())
    }
}

impl FromFolds for Springs {
    fn from_folds(s: &str, folds: u8) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let lines = s
            .split("\n")
            .filter(|line| line.trim().len() > 0)
            .collect::<Vec<_>>();

        let rows = lines
            .iter()
            .copied()
            .flat_map(|line| SpringRow::from_folds(line, folds))
            .collect::<Vec<_>>();

        Ok(Springs { rows })
    }
}
