use crate::lava_pattern::thing::Thing;
use row::PatternRow;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

mod row;
mod thing;

#[derive(Debug, Clone)]
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

    pub fn fix_smudge(&self) -> Result<((usize, usize), bool), ()> {
        let mirror_position =
            if let Some(&position) = self.mirror_position().ok_or(())?.iter().nth(0) {
                position
            } else {
                let rotated = self.rotate_90().unwrap();
                *rotated.mirror_position().unwrap().iter().nth(0).unwrap()
            };
        let height = self.0.len();
        let width = self.0.get(0).ok_or(())?.len();

        let mut new_mirror = None;
        for y in 0..height {
            for x in 0..width {
                let mut try_smudge = self.clone();
                let cell = try_smudge.0.get_mut(y).unwrap().get_mut(x).unwrap();
                println!(
                    "{:?} -> {y} {x} {cell}, {height}, {height}",
                    &self as *const _
                );

                match cell {
                    Thing::Rock => *cell = Thing::Ash,
                    Thing::Ash => *cell = Thing::Rock,
                }

                // try vertical
                if let Some(&new_position) = try_smudge.mirror_position().unwrap().iter().nth(0) {
                    if new_position != mirror_position {
                        new_mirror = Some((new_position, false));
                        break;
                    }
                }
                // try horizontal
                let rotated_try_smudge = try_smudge.rotate_90().unwrap();
                if let Some(&new_position) =
                    rotated_try_smudge.mirror_position().unwrap().iter().nth(0)
                {
                    if new_position != mirror_position {
                        new_mirror = Some((new_position, true));
                        break;
                    }
                }
            }
        }

        if new_mirror.is_none() {
            println!("{}", self);
            assert!(new_mirror.is_some());
        }
        new_mirror.ok_or(())
    }
}
