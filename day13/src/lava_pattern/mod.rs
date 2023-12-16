use row::PatternRow;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

mod row;
mod thing;

#[derive(Debug)]
pub struct LavaPattern(Vec<PatternRow>);

pub trait Mirror {
    fn mirror_position(&self) -> Option<HashSet<(usize, usize)>>;
}

impl Mirror for LavaPattern {
    fn mirror_position(&self) -> Option<HashSet<(usize, usize)>> {
        let mirrors = self
            .0
            .iter()
            .flat_map(PatternRow::mirror_position)
            .collect::<Vec<_>>();
        let potential_mirrors = mirrors.get(0)?;
        let result = potential_mirrors
            .iter()
            .filter(|&&potential_mirror| {
                mirrors
                    .iter()
                    .skip(1)
                    .all(|mirrors| mirrors.iter().any(|&mirror| mirror == potential_mirror))
            })
            .copied()
            .collect::<HashSet<_>>();

        Some(result)
    }
}

impl Display for LavaPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .0
            .iter()
            .map(|row| format!("{}\n", row))
            .collect::<String>();
        write!(f, "{}", str)
    }
}

impl FromStr for LavaPattern {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .split("\n")
            .filter(|line| line.trim().len() > 0)
            .collect::<Vec<_>>();

        let rows = lines
            .iter()
            .copied()
            .flat_map(PatternRow::from_str)
            .collect::<Vec<_>>();

        Ok(LavaPattern(rows))
    }
}

impl LavaPattern {
    pub fn rotate_90(&self) -> Result<Self, ()> {
        let height = self.0.len();
        let width = self.0.get(0).ok_or(())?.len();

        let rotated = (0..width)
            .map(|i| {
                (0..height)
                    .flat_map(move |j| self.0.get(j)?.get(i).copied())
                    .collect::<Vec<_>>()
                    .into()
            })
            .collect();

        Ok(LavaPattern(rotated))
    }
}
