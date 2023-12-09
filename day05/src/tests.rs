#[cfg(test)]
mod tests {
    use crate::{solution_01, solution_02, solution_02_bf};

    #[test]
    fn example_01() {
        let result = solution_01::<u8>(INPUT_EXAMPLE);

        assert_eq!(result, 35);
    }

    #[test]
    fn example_02() {
        let result = solution_02(INPUT_EXAMPLE);

        assert_eq!(result, 46);
    }

    #[test]
    fn example_02_bf() {
        let result = solution_02_bf(INPUT_EXAMPLE);

        assert_eq!(result, 46);
    }

    const INPUT_EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
}
