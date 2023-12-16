use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Thing {
    Rock,
    Ash,
}

impl FromStr for Thing {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            1 => match s.chars().nth(0).ok_or(())? {
                '.' => Ok(Self::Ash),
                '#' => Ok(Self::Rock),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Rock => "#",
            Self::Ash => ".",
        };
        write!(f, "{}", str)
    }
}
