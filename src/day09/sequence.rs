use std::fmt::Debug;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Sequence<T>
where
    T: Sub<Output = T>,
{
    sequence: Vec<T>,
    interpolation: Option<Box<Sequence<T>>>,
}

impl<T> Sequence<T>
where
    T: Sub<Output = T> + Copy + Add<Output = T> + Default,
{
    pub fn predict_forward(&self) -> T {
        match &self.interpolation {
            Some(interpolation) => *self.sequence.last().unwrap() + interpolation.predict_forward(),
            None => Default::default(),
        }
    }

    pub fn predict_backward(&self) -> T {
        match &self.interpolation {
            Some(interpolation) => {
                *self.sequence.first().unwrap() - interpolation.predict_backward()
            }
            None => Default::default(),
        }
    }
}

impl<T> From<Vec<T>> for Sequence<T>
where
    T: Sub<Output = T> + Copy + PartialEq + Default + Debug,
{
    fn from(sequence: Vec<T>) -> Self {
        let mut interpolation_vec = vec![];
        sequence
            .iter()
            .fold(None, |prev: Option<&T>, v| match prev {
                Some(pv) => {
                    interpolation_vec.push(*v - *pv);
                    Some(v)
                }
                None => Some(v),
            });

        let interpolation = if sequence.iter().all(|v| *v == Default::default()) {
            None
        } else {
            Some(Box::new(Sequence::from(interpolation_vec)))
        };

        Sequence {
            sequence,
            interpolation,
        }
    }
}
