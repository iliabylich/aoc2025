use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let machines = input.lines().map(Machine::parse).collect::<Vec<_>>();
    let mut sum = 0;
    for machine in machines {
        let num = machine.min_number_of_toggles();
        sum += num;
    }

    sum
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Buttons(HashSet<usize>);
impl Buttons {
    fn symmetric_difference(&self, other: &Self) -> Self {
        Self(self.0.symmetric_difference(&other.0).copied().collect())
    }
}
impl std::hash::Hash for Buttons {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for item in &self.0 {
            item.hash(state);
        }
    }
}

#[derive(Debug)]
struct Machine {
    start: Buttons,
    buttons: Vec<Buttons>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut it = line.split(" ");
        let first = it.next().unwrap();
        let mut rest = it.collect::<Vec<_>>();
        let _last = rest.pop().unwrap();

        let start = first
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| match c {
                '#' => Some(idx),
                '.' => None,
                _ => panic!("unknown char {c}"),
            })
            .collect::<HashSet<_>>();

        fn parse_button(s: &str) -> Buttons {
            let set = s
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<_>>();
            Buttons(set)
        }
        let buttons = rest.into_iter().map(parse_button).collect::<Vec<_>>();

        Machine {
            start: Buttons(start),
            buttons,
        }
    }

    fn min_number_of_toggles(self) -> u64 {
        let mut pool = HashMap::<Buttons, u64>::new();
        pool.insert(self.start, 0);

        let empty = Buttons(HashSet::new());

        while !pool.contains_key(&empty) {
            let mut newpool = HashMap::new();

            for (lhs, num) in pool.iter() {
                for rhs in self.buttons.iter() {
                    let next = lhs.symmetric_difference(rhs);
                    newpool.insert(next, *num + 1);
                }
            }

            pool = newpool;
        }

        *pool.get(&empty).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 7)
}
