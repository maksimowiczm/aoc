#[macro_export]
macro_rules! camel_groups_matching {
    ($groups:tt, $result:expr, $group_length:expr, $str:expr) => {{
        $groups
            .iter()
            .filter(|(_, cards)| cards.len() == $group_length)
            .map(|(_, _)| $result($str))
            .collect::<Vec<_>>()
    }};
}

#[macro_export]
macro_rules! value_function {
    ($fn_name:tt, $wrapper:ident) => {
        fn $fn_name(&self) -> Option<&str> {
            match self {
                $wrapper(s) => Some(s),
                _ => None,
            }
        }
    };
}
