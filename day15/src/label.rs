use std::str::FromStr;

pub struct Label(String);

impl FromStr for Label {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Label(s.to_string()))
    }
}

impl Label {
    pub fn hash(&self) -> u64 {
        self.0
            .chars()
            .fold(0, |acc, ch| (acc + ch as u64) * 17 % 256)
    }
}
