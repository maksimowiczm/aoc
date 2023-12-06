mod tests;

struct Race<'a>(&'a u64, &'a u64);

impl Race<'_> {
    fn possible_wins(&self) -> u64 {
        let time = *self.0;
        let distance = *self.1;

        (1..time).fold(0, |acc, velocity| {
            let time_left = time - velocity;
            if time_left * velocity > distance {
                acc + 1
            } else {
                acc
            }
        })
    }
}

pub fn solution_01(input: &str) -> u64 {
    let lines = input.split("\n").collect::<Vec<_>>();
    let times = lines[0]
        .chars()
        .skip(5)
        .collect::<String>()
        .trim()
        .split(" ")
        .flat_map(|v| v.parse::<u64>())
        .collect::<Vec<_>>();

    let distance = lines[1]
        .chars()
        .skip(9)
        .collect::<String>()
        .trim()
        .split(" ")
        .flat_map(|v| v.parse::<u64>())
        .collect::<Vec<_>>();

    assert_eq!(distance.len(), times.len());

    (0..times.len())
        .map(|i| Race(&times[i], &distance[i]))
        .fold(1, |acc, race| acc * race.possible_wins())
}
