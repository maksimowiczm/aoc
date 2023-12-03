use crate::board::Board;

mod board;
mod tests;

pub fn solution_01(input: &str) -> Option<u128> {
    let board: Board = input.into();

    Some(
        board
            .valid_part_numbers()
            .iter()
            .fold(0, |acc, v| acc + *v as u128),
    )
}
