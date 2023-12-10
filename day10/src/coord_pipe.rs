use crate::next_pipe::NextPipe;
use itertools::Itertools;
use num::{CheckedAdd, CheckedSub};

#[derive(Debug)]
pub struct CoordPipe<T> {
    symbol: char,
    location: (T, T),
}

impl<T> From<(char, (T, T))> for CoordPipe<T> {
    fn from((symbol, location): (char, (T, T))) -> Self {
        CoordPipe { symbol, location }
    }
}

impl<T> CoordPipe<T> {
    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn get_location(&self) -> &(T, T) {
        &self.location
    }
}

impl<T> CoordPipe<T>
where
    T: CheckedAdd + CheckedSub + From<i64> + PartialEq,
    CoordPipe<T>: NextPipe<T>,
{
    fn check_and_move(
        &self,
        from: &(T, T),
        actions: Vec<((T, T), fn(&CoordPipe<T>) -> Option<(T, T)>)>,
    ) -> Option<(T, T)> {
        if let Some((_, direction)) = actions.iter().find_or_last(|(vector, _)| {
            if let Some((x, y)) = move_by_vector(from, vector) {
                x == self.location.0 && y == self.location.1
            } else {
                false
            }
        }) {
            direction(self)
        } else {
            None
        }
    }

    fn top(&self) -> Option<(T, T)> {
        match self.symbol {
            '|' | 'L' | 'J' => move_by_vector(&self.location, &(T::from(0), T::from(-1))),
            _ => None,
        }
    }

    fn bottom(&self) -> Option<(T, T)> {
        match self.symbol {
            '|' | '7' | 'F' => move_by_vector(&self.location, &(T::from(0), T::from(1))),
            _ => None,
        }
    }

    fn right(&self) -> Option<(T, T)> {
        match self.symbol {
            '-' | 'L' | 'F' => move_by_vector(&self.location, &(T::from(1), T::from(0))),
            _ => None,
        }
    }

    fn left(&self) -> Option<(T, T)> {
        match self.symbol {
            '-' | 'J' | '7' => move_by_vector(&self.location, &(T::from(-1), T::from(0))),
            _ => None,
        }
    }
}

fn move_by_vector<T>(point: &(T, T), vector: &(T, T)) -> Option<(T, T)>
where
    T: CheckedAdd + CheckedSub + PartialEq + From<i64>,
{
    let (x, y) = point;
    let (vx, vy) = vector;
    let out = (x.checked_add(vx), y.checked_add(vy));
    match out {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

impl<T> NextPipe<T> for CoordPipe<T>
where
    T: CheckedAdd + CheckedSub + PartialEq + From<i64>,
{
    fn next(&self, from: &(T, T)) -> Option<(T, T)> {
        match self.symbol {
            '|' => self.check_and_move(
                from,
                vec![
                    ((T::from(0), T::from(-1)), Self::top),
                    ((T::from(0), T::from(1)), Self::bottom),
                ],
            ),
            '-' => self.check_and_move(
                from,
                vec![
                    ((T::from(-1), T::from(0)), Self::left),
                    ((T::from(1), T::from(0)), Self::right),
                ],
            ),
            'L' => self.check_and_move(
                from,
                vec![
                    ((T::from(-1), T::from(0)), Self::top),
                    ((T::from(0), T::from(1)), Self::right),
                ],
            ),
            'J' => self.check_and_move(
                from,
                vec![
                    ((T::from(1), T::from(0)), Self::top),
                    ((T::from(0), T::from(1)), Self::left),
                ],
            ),
            '7' => self.check_and_move(
                from,
                vec![
                    ((T::from(1), T::from(0)), Self::bottom),
                    ((T::from(0), T::from(-1)), Self::left),
                ],
            ),
            'F' => self.check_and_move(
                from,
                vec![
                    ((T::from(-1), T::from(0)), Self::bottom),
                    ((T::from(0), T::from(-1)), Self::right),
                ],
            ),
            _ => None,
        }
    }
}
