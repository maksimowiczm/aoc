#[cfg(test)]
mod tests {
    use crate::solution_01;

    #[test]
    fn example_01_01() {
        const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = solution_01(INPUT);

        assert_eq!(result, Ok(1320));
    }
}
