use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl FromStr for Spring {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            1 => match s.chars().nth(0).ok_or(())? {
                '.' => Ok(Self::Operational),
                '#' => Ok(Self::Damaged),
                '?' => Ok(Self::Unknown),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl ToString for Spring {
    fn to_string(&self) -> String {
        match self {
            Self::Operational => ".".to_owned(),
            Self::Damaged => "#".into(),
            Self::Unknown => "?".to_string(),
        }
    }
}
