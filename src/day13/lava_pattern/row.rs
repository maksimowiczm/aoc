use super::Mirror;
use crate::day13::lava_pattern::thing::Thing;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PatternRow(Vec<Thing>);

impl FromStr for PatternRow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let things = s
            .chars()
            .flat_map(|ch| Thing::from_str(ch.to_string().as_str()))
            .collect();

        Ok(PatternRow(things))
    }
}

impl Display for PatternRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(Thing::to_string).collect::<String>()
        )
    }
}

impl From<Vec<Thing>> for PatternRow {
    fn from(value: Vec<Thing>) -> Self {
        PatternRow(value)
    }
}

impl PatternRow {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, i: usize) -> Option<&Thing> {
        self.0.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Thing> {
        self.0.get_mut(i)
    }
}

fn is_mirror(things: &Vec<&Thing>) -> bool {
    let reversed = things.iter().rev();
    let matching = reversed
        .zip(things.iter())
        .filter(|&(a, b)| *a == *b)
        .count();

    matching == things.len()
}

impl Mirror for PatternRow {
    fn mirror_position(&self) -> Option<HashSet<(usize, usize)>> {
        let width = self.len();

        let result = (0..width)
            .flat_map(|x| {
                (1..x + 1)
                    .map(move |i| {
                        // create subpattern x as center i * 2 as width
                        let sub_pattern = self.0.iter().skip(x - i).take(i * 2).collect::<Vec<_>>();
                        (x, sub_pattern)
                    })
                    // take only patterns with center and mirrors
                    .filter(|(_, sub_pattern)| sub_pattern.len() % 2 == 0 && is_mirror(sub_pattern))
                    // take only edge mirrors
                    .filter(|(x, sub_pattern)| {
                        x - sub_pattern.len() / 2 == 0 || x + sub_pattern.len() / 2 == self.len()
                    })
                    .map(|(x, _)| (x - 1, x))
                    .collect::<Vec<_>>()
            })
            .collect();

        Some(result)
    }
}
