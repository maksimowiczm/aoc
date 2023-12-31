use dish::Dish;

mod dish;
mod tests;
mod aoc_tests;

fn solution_01(input: &str) -> Result<u64, ()> {
    let dish = input.parse::<Dish>()?;

    dish.tilt_north_load()
}
