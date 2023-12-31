use crate::lens::{Lens, LensOperation};
use std::collections::HashMap;

pub struct Book(HashMap<u8, Vec<Lens>>);

impl Book {
    pub fn new() -> Self {
        Book(HashMap::new())
    }

    pub fn execute_sequence(&mut self, sequence: &Vec<LensOperation>) {
        sequence.iter().for_each(|operation| match operation {
            LensOperation::Dash(label) => {
                let hash = u8::try_from(label.hash()).unwrap();
                self.0.entry(hash).and_modify(|lens_box| {
                    if let Some(index) = lens_box.iter().position(|lens| lens.label == *label) {
                        lens_box.remove(index);
                    }
                });
            }
            LensOperation::Equals(lens) => {
                let hash = u8::try_from(lens.label.hash()).unwrap();
                let lens_box = self.0.entry(hash).or_insert(vec![]);
                let index = lens_box
                    .iter()
                    .position(|boxed_lens| lens.label == boxed_lens.label);

                lens_box.push(lens.clone());

                if let Some(index) = index {
                    lens_box.swap_remove(index);
                }
            }
        })
    }

    pub fn get_focusing_power(&self) -> u64 {
        self.0
            .iter()
            .filter(|(_, vec)| !vec.is_empty())
            .map(|(&box_number, &ref lens_box)| {
                lens_box.iter().enumerate().fold(0, |acc, (i, lens)| {
                    acc + (box_number + 1) as u64 * (i + 1) as u64 * lens.focal
                })
            })
            .sum()
    }
}
