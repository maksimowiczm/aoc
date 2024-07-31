use crate::solution::SolveDay;
use num_integer::lcm;
use std::collections::{HashMap, HashSet, VecDeque};

pub(crate) struct Day20;

impl SolveDay for Day20 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        Some(part1(input))
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        Some(part2(input))
    }
}

fn part1(input: &str) -> u64 {
    let mut circuit = Circuit::parse(input);

    let mut low_count = 0;
    let mut high_count = 0;

    const INITIAL_PULSE: Pulse = Pulse {
        from: "me",
        to: "broadcaster",
        kind: PulseKind::Low,
    };

    for _ in 0..1000 {
        let mut queue = VecDeque::from([INITIAL_PULSE]);
        while let Some(pulse) = queue.pop_front() {
            match pulse.kind {
                PulseKind::Low => low_count += 1,
                PulseKind::High => high_count += 1,
            }
            _ = circuit.send(pulse, &mut queue);
        }
    }

    low_count * high_count
}

fn part2(input: &str) -> u64 {
    let mut circuit = Circuit::parse(input);

    // get all inputs to conjunction with rx
    let connector = *circuit
        .connections
        .iter()
        .find(|(_, connections)| connections.iter().any(|connection| connection == &"rx"))
        .unwrap()
        .0;

    let mut inputs = circuit
        .connections
        .iter()
        .filter(|(_, connections)| {
            connections
                .iter()
                .any(|connection| *connection == connector)
        })
        .map(|(input, _)| *input)
        .collect::<HashSet<_>>();

    const INITIAL_PULSE: Pulse = Pulse {
        from: "me",
        to: "broadcaster",
        kind: PulseKind::Low,
    };

    // count when connector with rx as output will be low,
    // this is the time when all inputs are high
    let mut when = vec![];
    let mut iteration = 0;
    loop {
        iteration += 1;
        let mut queue = VecDeque::from([INITIAL_PULSE]);
        while let Some(pulse) = queue.pop_front() {
            if pulse.kind == PulseKind::High
                && pulse.to == connector
                && inputs.contains(&pulse.from)
            {
                when.push(iteration);
                inputs.remove(pulse.from);

                if inputs.is_empty() {
                    // connector will be low if all inputs are high at once, lcm
                    return when.into_iter().fold(1, lcm);
                }
            }

            circuit.send(pulse, &mut queue);
        }
    }
}

#[allow(dead_code)]
fn part2_bf(input: &str) -> u64 {
    let mut circuit = Circuit::parse(input);

    const INITIAL_PULSE: Pulse = Pulse {
        from: "me",
        to: "broadcaster",
        kind: PulseKind::Low,
    };

    let mut iteration = 0;
    loop {
        iteration += 1;

        if iteration % 1_000_000 == 0 {
            println!("Iteration: {}", iteration);
        }

        let mut queue = VecDeque::from([INITIAL_PULSE]);
        while let Some(pulse) = queue.pop_front() {
            if pulse.to == "rx" && pulse.kind == PulseKind::Low {
                return iteration;
            }
            circuit.send(pulse, &mut queue);
        }
    }
}

struct Circuit<'a> {
    connectors: HashMap<&'a str, Connector<'a>>,
    connections: HashMap<&'a str, Vec<&'a str>>,
}

#[derive(Debug, PartialEq)]
enum Connector<'memo> {
    FlipFlop {
        state: bool,
    },
    Conjunction {
        memory: HashMap<&'memo str, PulseKind>,
    },
    Broadcaster,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PulseKind {
    High,
    Low,
}

#[derive(Debug)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    kind: PulseKind,
}

impl Circuit<'_> {
    fn parse(input: &str) -> Circuit {
        let mut connectors = HashMap::new();
        let mut connections = HashMap::new();

        for line in input.lines() {
            let (name, connector, output) = parse_line(line);
            connectors.insert(name, connector);
            connections.insert(name, output);
        }

        for (from, to) in &connections {
            for connection in to {
                if let Some(Connector::Conjunction { memory }) = connectors.get_mut(*connection) {
                    memory.insert(from, PulseKind::Low);
                }
            }
        }

        Circuit {
            connectors,
            connections,
        }
    }
}

impl<'a> Circuit<'a> {
    fn send(&mut self, pulse: Pulse<'a>, queue: &mut VecDeque<Pulse<'a>>) {
        let Some(connector) = self.connectors.get_mut(pulse.to) else {
            return;
        };

        let pulse_kind = match connector {
            Connector::Broadcaster => Some(PulseKind::Low),
            Connector::FlipFlop { state } => match pulse.kind {
                PulseKind::High => None,
                PulseKind::Low => {
                    *state = !*state;
                    match state {
                        true => Some(PulseKind::High),
                        false => Some(PulseKind::Low),
                    }
                }
            },
            Connector::Conjunction { memory } => {
                memory
                    .get_mut(pulse.from)
                    .map(|kind| {
                        *kind = pulse.kind;
                    })
                    .unwrap();

                if memory.values().all(|kind| *kind == PulseKind::High) {
                    Some(PulseKind::Low)
                } else {
                    Some(PulseKind::High)
                }
            }
        };

        if let Some(pulse_kind) = pulse_kind {
            for connection in self.connections.get(pulse.to).unwrap() {
                let pulse = Pulse {
                    from: pulse.to,
                    to: connection,
                    kind: pulse_kind,
                };
                queue.push_back(pulse);
            }
        }
    }
}

fn parse_line(input: &str) -> (&str, Connector, Vec<&str>) {
    let (lhs, rhs) = input.split_once(" -> ").unwrap();
    let module = if lhs.starts_with("%") {
        Connector::FlipFlop { state: false }
    } else if lhs.starts_with("&") {
        Connector::Conjunction {
            memory: HashMap::new(),
        }
    } else if lhs == "broadcaster" {
        Connector::Broadcaster
    } else {
        panic!("Invalid input")
    };

    let name = if module == Connector::Broadcaster {
        lhs
    } else {
        &lhs[1..]
    };

    let rhs = rhs.split(", ").collect();
    (name, module, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(part1(INPUT), 32000000);
    }

    #[test]
    fn example_02_bf() {
        const INPUT: &str = "broadcaster -> b, c
%b -> a
%c -> a
&a -> rx";

        assert_eq!(part2_bf(INPUT), 1);
    }

    #[test]
    fn example_02() {
        const INPUT: &str = "broadcaster -> b, c
%b -> a
%c -> a
&a -> rx";

        assert_eq!(part2(INPUT), 1);
    }
}
