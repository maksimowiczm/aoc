use crate::day16::cave::Cave;
use std::fmt::Display;
use std::vec;

#[derive(Clone)]
pub enum BeamCell {
    Energized(Vec<BeamDirection>),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BeamDirection {
    Up,
    Left,
    Down,
    Right,
}

pub struct Beam(Vec<Vec<BeamCell>>);

#[derive(Debug, Clone, Copy, PartialEq)]
struct BeamHead {
    x: u8,
    y: u8,
    direction: BeamDirection,
}

impl BeamHead {
    fn new(x: u8, y: u8, direction: BeamDirection) -> BeamHead {
        BeamHead { x, y, direction }
    }

    fn beamize(&self, cave: &Cave) -> Option<Vec<BeamHead>> {
        let cell = &cave.0[self.y as usize][self.x as usize];
        let result = cell
            .get_next_directions(self.direction)
            .iter()
            .map(|direction| BeamHead::new(self.x, self.y, *direction))
            .collect::<Vec<_>>();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn next(&mut self, cave: &Cave) -> Result<(), ()> {
        match self.direction {
            BeamDirection::Up => self.y = self.y.checked_sub(1).ok_or(())?,
            BeamDirection::Left => self.x = self.x.checked_sub(1).ok_or(())?,
            BeamDirection::Down => self.y = self.y + 1,
            BeamDirection::Right => self.x = self.x + 1,
        }

        if (self.x as usize) < cave.0[0].len() && (self.y as usize) < cave.0.len() {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Beam {
    fn add_beam(&mut self, beam: &BeamHead) -> Result<(), ()> {
        let (x, y) = (beam.x as usize, beam.y as usize);
        let cell = self.0.get_mut(y).ok_or(())?.get_mut(x).ok_or(())?;
        match cell {
            BeamCell::Energized(vec) => {
                if vec.iter().position(|dir| *dir == beam.direction).is_some() {
                    Err(())
                } else {
                    vec.push(beam.direction.clone());
                    Ok(())
                }
            }
            BeamCell::Empty => {
                *cell = BeamCell::Energized(vec![beam.direction.clone()]);
                Ok(())
            }
        }
    }

    pub fn count_energized(&self) -> u64 {
        self.0
            .iter()
            .flatten()
            .map(|cell| match cell {
                BeamCell::Energized(_) => 1,
                _ => 0,
            })
            .sum()
    }

    pub fn from_cave_with_start(cave: &Cave, (x, y, direction): (u8, u8, BeamDirection)) -> Self {
        let mut beam_grid = Beam(vec![vec![BeamCell::Empty; cave.0[0].len()]; cave.0.len()]);
        let start = BeamHead { x, y, direction };
        let beamized_start = start.beamize(&cave);
        let mut beams = match beamized_start {
            Some(vec) => vec,
            _ => vec![start],
        };

        loop {
            let iteration = beams
                .iter_mut()
                .flat_map(|beam| {
                    if beam_grid.add_beam(beam).is_err() {
                        return Some((*beam, vec![]));
                    }

                    let to_delete = if beam.next(&cave).is_err() {
                        Some(*beam)
                    } else {
                        None
                    };

                    if beam.x as usize >= cave.0[0].len() || beam.y as usize >= cave.0.len() {
                        return None;
                    }

                    let beamize = beam.beamize(&cave);
                    match beamize {
                        Some(heads) => Some((*beam, heads)),
                        None => match to_delete {
                            Some(beam) => Some((beam, vec![])),
                            None => None,
                        },
                    }
                })
                .collect::<Vec<_>>();

            iteration.iter().for_each(|(to_delete, to_add)| {
                beams.extend(to_add);
                beams.swap_remove(beams.iter().position(|b| b == to_delete).unwrap());
            });

            // println!("{beam_grid}");
            // println!();
            if beams.is_empty() {
                break;
            }
        }

        beam_grid
    }

    pub fn from_cave(cave: &Cave) -> Self {
        Self::from_cave_with_start(cave, (0, 0, BeamDirection::Right))
    }
}

impl Display for BeamCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeamCell::Energized(_) => write!(f, "#"),
            BeamCell::Empty => write!(f, "."),
        }
    }
}

impl Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .0
            .iter()
            .map(|row| row.iter().map(|cell| format!("{cell}")).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", str)
    }
}
