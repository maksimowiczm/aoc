use std::{str::FromStr, vec};

use crate::beam::BeamDirection;

#[derive(Debug)]
pub enum Cell {
    MirrorLeft,
    MirrorRight,
    SplitterHorizontal,
    SplitterVertical,
    Empty,
}

impl Cell {
    pub fn get_next_directions(&self, direction: BeamDirection) -> Vec<BeamDirection> {
        match self {
            Cell::MirrorRight => match direction {
                BeamDirection::Right => vec![BeamDirection::Up],
                BeamDirection::Down => vec![BeamDirection::Left],
                BeamDirection::Left => vec![BeamDirection::Down],
                BeamDirection::Up => vec![BeamDirection::Right],
            },
            Cell::MirrorLeft => match direction {
                BeamDirection::Right => vec![BeamDirection::Down],
                BeamDirection::Down => vec![BeamDirection::Right],
                BeamDirection::Left => vec![BeamDirection::Up],
                BeamDirection::Up => vec![BeamDirection::Left],
            },
            Cell::SplitterHorizontal => match direction {
                BeamDirection::Left | BeamDirection::Right => vec![],
                BeamDirection::Up | BeamDirection::Down => {
                    vec![BeamDirection::Left, BeamDirection::Right]
                }
            },
            Cell::SplitterVertical => match direction {
                BeamDirection::Left | BeamDirection::Right => {
                    vec![BeamDirection::Up, BeamDirection::Down]
                }
                BeamDirection::Up | BeamDirection::Down => {
                    vec![]
                }
            },
            Cell::Empty => vec![],
        }
    }
}

pub struct Cave(pub Vec<Vec<Cell>>);

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<_>>();
        let cave = lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Cell::Empty,
                        '/' => Cell::MirrorRight,
                        '\\' => Cell::MirrorLeft,
                        '|' => Cell::SplitterVertical,
                        '-' => Cell::SplitterHorizontal,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Cave(cave))
    }
}
