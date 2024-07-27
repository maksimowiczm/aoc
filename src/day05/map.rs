use std::ops::{Add, Sub};
use std::str::FromStr;

pub struct Map<T> {
    destination: T,
    source: T,
    range: T,
}

impl<T> FromStr for Map<T>
where
    T: FromStr + Clone,
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

pub struct CategoryMap<'a, T> {
    maps: &'a Vec<Map<T>>,
}

impl<'a, T> From<&'a Vec<Map<T>>> for CategoryMap<'a, T> {
    fn from(maps: &'a Vec<Map<T>>) -> Self {
        CategoryMap { maps }
    }
}

impl<T> CategoryMap<'_, T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Clone,
{
    pub fn convert_to_destination(&self, value: T) -> T {
        let map_for_value = self
            .maps
            .iter()
            .find(|m| m.source <= value && value < m.source.clone() + m.range.clone());

        if let Some(map) = map_for_value {
            map.destination.clone() + value - map.source.clone()
        } else {
            value
        }
    }

    pub fn convert_to_source(&self, value: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Ord + Copy,
    {
        let map_for_value = self
            .maps
            .iter()
            .find(|m| m.destination <= value && value < (m.destination + m.range));

        if let Some(map) = map_for_value {
            map.source + value - map.destination
        } else {
            value
        }
    }
}
