use itertools::Itertools;
use rand::Rng;

use crate::springs::spring::Spring;
use crate::springs::Arrangements;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec;

#[derive(Debug)]
pub struct SpringRow {
    row: Vec<Spring>,
    groups: Vec<u64>,
}

impl FromStr for SpringRow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim().split_whitespace().collect::<Vec<_>>();

        let row = input.get(0).ok_or(())?.to_string();
        let row = row
            .chars()
            .flat_map(|ch| Spring::from_str(ch.to_string().as_str()))
            .collect();

        let groups = input.get(1).ok_or(())?;
        let groups = groups.split(",").flat_map(u64::from_str).collect();

        Ok(SpringRow { row, groups })
    }
}

impl ToString for SpringRow {
    fn to_string(&self) -> String {
        self.row.iter().map(|spring| spring.to_string()).collect()
    }
}

impl Arrangements for SpringRow {
    fn arrangements_count(&self) -> u64 {
        let (_, damaged_count, unknown_count) = self.count_springs();

        let groups_sum = self.groups.iter().sum::<u64>();
        let to_randomize = groups_sum - damaged_count;
        let randomize_count = num_integer::binomial(unknown_count, to_randomize);

        let unknown_indexes = self
            .row
            .iter()
            .enumerate()
            .filter(|(_, spring)| match spring {
                Spring::Unknown => true,
                _ => false,
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        // XD 💀💀💀💀💀💀💀💀💀💀💀💀💀
        let mut rng = rand::thread_rng();
        let mut already_checked = HashMap::<Vec<u64>, bool>::new();
        (0..randomize_count).for_each(|_| loop {
            let mut indexes = vec![];

            loop {
                if indexes.len() as u64 == to_randomize {
                    break;
                }
                let index = rng.gen_range(0..unknown_count);
                if indexes
                    .iter()
                    .all(|v| *v != unknown_indexes[index as usize] as u64)
                {
                    indexes.push(unknown_indexes[index as usize] as u64);
                }
            }

            indexes.sort();
            if already_checked.contains_key(&indexes) {
                continue;
            } else {
                already_checked.insert(indexes, true);
                break;
            }
        });

        let rows = already_checked
            .keys()
            .map(|indexes| {
                let mut row = self.row.clone();

                row.iter_mut()
                    .filter(|spring| match spring {
                        Spring::Unknown => true,
                        _ => false,
                    })
                    .for_each(|spring| *spring = Spring::Operational);

                indexes.iter().for_each(|&i| {
                    row[i as usize] = Spring::Damaged;
                });

                row
            })
            .collect::<Vec<_>>();

        rows.iter().fold(
            0,
            |acc, row| {
                if self.valid_row(&row) {
                    acc + 1
                } else {
                    acc
                }
            },
        )
    }
}

impl SpringRow {
    fn count_springs(&self) -> (u64, u64, u64) {
        self.row.iter().fold(
            (0, 0, 0),
            |(operational, damaged, unknown), spring| match spring {
                Spring::Operational => (operational + 1, damaged, unknown),
                Spring::Damaged => (operational, damaged + 1, unknown),
                Spring::Unknown => (operational, damaged, unknown + 1),
            },
        )
    }

    pub fn valid_row(&self, row: &Vec<Spring>) -> bool {
        // not valid if any spring is unknown
        if row.iter().any(|spring| match spring {
            Spring::Unknown => true,
            _ => false,
        }) {
            return false;
        }

        // group by damaged springs
        let grouped_springs = row
            .iter()
            .group_by(|key| *key)
            .into_iter()
            .map(|(spring, group)| (spring, group.count()))
            .filter(|(spring, _)| match spring {
                Spring::Damaged => true,
                _ => false,
            })
            .map(|(_, count)| count as u64)
            .collect::<Vec<_>>();

        // match groups with groups patter
        let matching = self
            .groups
            .iter()
            .zip(&grouped_springs)
            .filter(|&(a, b)| a == b)
            .count();

        matching == self.groups.len()
    }
}
