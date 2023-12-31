use std::str::FromStr;

use crate::label::Label;

#[derive(Clone)]
pub struct Lens {
    pub label: Label,
    pub focal: u64,
}

pub enum LensOperation {
    Dash(Label),
    Equals(Lens),
}

impl FromStr for LensOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let label = s
            .chars()
            .take_while(|&ch| ch != '-' && ch != '=')
            .collect::<String>()
            .parse::<Label>()?;
        let operation = s
            .chars()
            .skip_while(|&ch| ch != '-' && ch != '=')
            .next()
            .ok_or(())?;

        match operation {
            '=' => {
                let focal = s
                    .chars()
                    .skip_while(|&ch| ch != '-' && ch != '=')
                    .skip(1)
                    .take(1)
                    .collect::<String>()
                    .parse::<u64>()
                    .or(Err(()))?;

                Ok(LensOperation::Equals(Lens { label, focal }))
            }
            '-' => Ok(LensOperation::Dash(label)),
            _ => panic!(),
        }
    }
}
