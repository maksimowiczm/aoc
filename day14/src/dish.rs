use core::panic;
use std::{iter, str::FromStr};

pub struct Dish(Vec<Vec<Option<Rock>>>);

#[derive(Clone)]
enum Rock {
    Rounded,
    Solid,
}

fn wrap<T>(vector: Vec<T>, wrapper: T) -> Vec<T>
where
    T: Clone,
{
    iter::once(wrapper.clone())
        .chain(vector)
        .chain(iter::once(wrapper.clone()))
        .collect()
}

impl FromStr for Dish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let lines = s.split("\n").filter(|l| !l.is_empty()).collect::<Vec<_>>();

        let rows = lines
            .iter()
            .map(|line| {
                wrap(
                    line.chars()
                        .map(|ch| match ch {
                            'O' => Some(Rock::Rounded),
                            '#' => Some(Rock::Solid),
                            '.' => None,
                            _ => panic!(),
                        })
                        .collect::<Vec<_>>(),
                    Some(Rock::Solid),
                )
            })
            .collect::<Vec<_>>();

        let width = rows.get(0).ok_or(())?.len();
        let wrapped = wrap(rows, vec![Some(Rock::Solid); width]);

        Ok(Dish(wrapped))
    }
}

impl Dish {
    pub fn tilt_north_load(&self) -> Result<u64, ()> {
        let height = self.0.len();
        let width = self.0.get(0).ok_or(())?.len();

        let columns = (0..width)
            .map(|i| (0..height).rev().map(|j| &self.0[j][i]).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let rounded_above_solid_row_and_count = columns
            .iter()
            .flat_map(|column| {
                let mut rounded_above = 0;
                column
                    .iter()
                    .enumerate()
                    .flat_map(move |(row, rock)| match rock {
                        None => None,
                        Some(Rock::Rounded) => {
                            rounded_above += 1;
                            None
                        }
                        Some(Rock::Solid) => {
                            let res = Some((row, rounded_above));
                            rounded_above = 0;
                            res
                        }
                    })
                    .filter(|(_, count)| *count > 0)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let result = rounded_above_solid_row_and_count
            .iter()
            .fold(0, |acc, &(row, count)| {
                acc + ((row - count)..row).sum::<usize>()
            });

        Ok(result as u64)
    }
}
