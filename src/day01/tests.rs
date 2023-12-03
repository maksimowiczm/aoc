use crate::day01::{day01_result_01, day01_result_02};

#[test]
fn example_01() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    unsafe {
        let result = day01_result_01(input.as_bytes().as_ptr(), input.len() as u64);
        assert_eq!(result, 142);
    }
}

#[test]
fn example_02() {
    let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    unsafe {
        let result = day01_result_02(input.as_bytes().as_ptr(), input.len() as u64);
        assert_eq!(result, 281);
    }
}
