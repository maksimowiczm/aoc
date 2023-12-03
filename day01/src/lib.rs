mod tests;

#[link(name = "day01", kind = "static")]
extern "C" {
    fn day01_result_01(input: *const u8, n: u64) -> u64;
    fn day01_result_02(input: *const u8, n: u64) -> u64;
}
