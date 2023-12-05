use std::ops::{Add, Sub};
use std::str::FromStr;

pub struct Map<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Clone,
{
    destination: T,
    source: T,
    range: T,
}

impl<T> FromStr for Map<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Clone,
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .trim()
            .split(" ")
            .map(|v| v.parse::<T>())
            .flatten()
            .collect::<Vec<_>>();

        if map.len() != 3 {
            return Err(());
        } else {
            Ok(Map {
                destination: map[0].clone(),
                source: map[1].clone(),
                range: map[2].clone(),
            })
        }
    }
}

pub struct CategoryMap<'a, T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Clone,
{
    from: String,
    to: String,
    pub(crate) maps: &'a Vec<Map<T>>,
}

impl<'a, T> From<(String, String, &'a Vec<Map<T>>)> for CategoryMap<'a, T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Clone,
{
    fn from((from, to, maps): (String, String, &'a Vec<Map<T>>)) -> Self {
        CategoryMap { from, to, maps }
    }
}

impl<T> CategoryMap<'_, T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Clone,
{
    pub fn convert(&self, value: T) -> T {
        let map_for_value = self
            .maps
            .iter()
            .find(|m| m.source <= value && value <= m.source.clone() + m.range.clone());

        if let Some(map) = map_for_value {
            map.destination.clone() + value - map.source.clone()
        } else {
            value
        }
    }
}
