mod tests;

#[link(name = "day02", kind = "static")]
extern "C" {
    fn day02_solution_01(input: *const u8, n: u64) -> u64;
    fn day02_solution_02(input: *const u8, n: u64) -> u64;
}
