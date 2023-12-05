use std::str::FromStr;

pub struct Map {
    source: u32,
    destination: u32,
    range: u32,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .trim()
            .split(" ")
            .map(|v| v.parse::<u32>())
            .flatten()
            .collect::<Vec<_>>();

        if map.len() != 3 {
            return Err(());
        } else {
            Ok(Map {
                source: map[0],
                destination: map[1],
                range: map[2],
            })
        }
    }
}

pub struct Category {
    pub(crate) from: String,
    pub(crate) to: String,
}

impl FromStr for Category {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .replace("map:", "")
            .split("-to-")
            .map(|i| i.trim().to_owned())
            .collect::<Vec<_>>();

        if map.len() != 2 {
            Err(())
        } else {
            Ok(Category {
                from: map[0].clone(),
                to: map[1].clone(),
            })
        }
    }
}

pub struct CategoryMap<'a> {
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) maps: &'a Vec<Map>,
}
