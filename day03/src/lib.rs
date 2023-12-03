use crate::board::Board;

mod board;
mod tests;

pub fn solution_01(input: &str) -> Option<u128> {
    let board: Board = input.into();

    Some(
        board
            .valid_parts()
            .iter()
            .fold(0, |acc, v| acc + *v as u128),
    )
}

pub fn solution_02(input: &str) -> Option<u128> {
    let board: Board = input.into();

    Some(
        board
            .valid_gears()
            .iter()
            .fold(0, |acc, v| acc + *v as u128),
    )
}
