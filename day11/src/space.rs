use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Space {
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    galaxies: Vec<(usize, usize)>,
}

impl Space {
    pub fn distance_between_points(
        &self,
        (from_row, from_col): (usize, usize),
        (to_row, to_col): (usize, usize),
    ) -> usize {
        let empty_rows = self
            .empty_rows
            .iter()
            .filter(|&&v| {
                if from_row < to_row {
                    from_row < v && v < to_row
                } else {
                    to_row < v && v < from_row
                }
            })
            .count();
        let distance_row = from_row.abs_diff(to_row) + empty_rows;

        let empty_cols = self
            .empty_cols
            .iter()
            .filter(|&&v| {
                if from_col < to_col {
                    from_col < v && v < to_col
                } else {
                    to_col < v && v < from_col
                }
            })
            .count();
        let distance_cols = from_col.abs_diff(to_col) + empty_cols;

        distance_cols + distance_row
    }

    pub fn distance_between_galaxies_pairs(&self) -> u64 {
        let distance = self
            .galaxies
            .iter()
            .map(|galaxy| {
                self.galaxies
                    .iter()
                    .filter(|g| *g != galaxy)
                    .map(|other| self.distance_between_points(*galaxy, *other))
                    .sum::<usize>()
            })
            .sum::<usize>();

        distance as u64 / 2
    }
}

impl FromStr for Space {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines_iter = input.split("\n");
        let mut cols = HashMap::<usize, usize>::new();

        let width = match lines_iter.clone().next() {
            Some(line) => line.len(),
            _ => 0,
        };

        let galaxies = lines_iter
            .clone()
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(i, line)| {
                let galaxies = line
                    .chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .collect::<Vec<_>>();

                galaxies.iter().for_each(|&(j, _)| {
                    cols.entry(j).and_modify(|v| *v += 1).or_insert(1);
                });

                match galaxies.len() {
                    0 => None,
                    _ => Some(galaxies.iter().map(|(j, _)| (i, *j)).collect::<Vec<_>>()),
                }
            })
            .collect::<Vec<_>>();

        let empty_rows = galaxies
            .iter()
            .enumerate()
            .filter(|(_, row)| row.is_none())
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let galaxies = galaxies.iter().flatten().flatten().copied().collect();

        let mut empty_cols = (0..width).collect::<Vec<_>>();
        cols.keys()
            .for_each(|col| match empty_cols.iter().position(|x| *x == *col) {
                Some(position) => {
                    empty_cols.remove(position);
                }
                _ => (),
            });

        Ok(Space {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}
