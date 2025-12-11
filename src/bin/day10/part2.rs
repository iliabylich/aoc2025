use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let machines = input.lines().map(Machine::parse).collect::<Vec<_>>();

    machines
        .into_iter()
        .map(|machine| {
            let sys = machine.into_equations();
            sys.min_solution().unwrap()
        })
        .sum()
}

#[derive(Debug)]
struct Machine {
    combinations: Vec<Vec<u8>>,
    target: Vec<u8>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut it = line.split(" ");
        let _first = it.next().unwrap();
        let mut rest = it.collect::<Vec<_>>();
        let last = rest.pop().unwrap();

        let target = last
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        fn parse_combination(s: &str) -> Vec<u8> {
            s.strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        }
        let combinations = rest.into_iter().map(parse_combination).collect::<Vec<_>>();

        Machine {
            combinations,
            target,
        }
    }

    fn into_equations(self) -> EqSystem {
        let mut equations = vec![];

        for (num_idx, num) in self.target.iter().copied().enumerate() {
            let total: u8 = num;
            let mut equation = Equation([false; 13], total);

            for (idx, combination) in self.combinations.iter().enumerate() {
                if combination.contains(&(u8::try_from(num_idx).unwrap())) {
                    assert!(idx < 13);
                    equation.0[idx] = true;
                }
            }

            equations.push(equation);
        }

        EqSystem {
            equations,
            substitutions: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Equation([bool; 13], u8);
impl Equation {
    fn optimize(&self) -> Option<(usize, u8)> {
        let mut idx = None;
        for i in 0..13 {
            if self.0[i] {
                if idx.is_some() {
                    return None;
                }
                idx = Some(i);
            }
        }
        Some((idx?, self.1))
    }
    fn is_completed(&self) -> bool {
        self.length() == 0 && self.1 == 0
    }
}
impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = vec![];
        for (idx, set) in self.0.iter().copied().enumerate() {
            if set {
                buf.push(format!("x{idx}"));
            }
        }
        write!(f, "{} = {}", buf.join(" + "), self.1)
    }
}
impl Equation {
    fn length(&self) -> usize {
        self.0.iter().filter(|e| **e).count()
    }
    fn substitute(&mut self, x: usize, value: u8) -> bool {
        assert!(x < 13);
        if self.0[x] {
            let Some(newtotal) = self.1.checked_sub(value) else {
                return false;
            };
            self.1 = newtotal;
            self.0[x] = false;

            if self.length() == 0 && self.1 != 0 {
                return false;
            }
        }
        true
    }
    fn assumption_to_make(&self) -> (usize, Range<u8>) {
        assert!(!self.is_completed());
        let Some((x, _)) = self.0.iter().enumerate().find(|&(_idx, &set)| set) else {
            panic!("Failed to find set X in {self}");
        };
        (x, 0..self.1 + 1)
    }
}

#[derive(Debug, Clone)]
struct EqSystem {
    equations: Vec<Equation>,
    substitutions: HashMap<usize, u8>,
}

impl EqSystem {
    fn best_assumption(&self) -> Option<(usize, Range<u8>)> {
        let (idx, _len) = self
            .equations
            .iter()
            .enumerate()
            .map(|(idx, eq)| (idx, eq.length()))
            .min_by_key(|&(_idx, len)| len)?;
        let eq = self.equations[idx];
        Some(eq.assumption_to_make())
    }

    fn substitute_and_optimize(&mut self, x: usize, value: u8) -> bool {
        struct Substite(usize, u8);

        let mut queue = VecDeque::new();
        queue.push_back(Substite(x, value));

        while let Some(Substite(x, value)) = queue.pop_front() {
            if !self.substitute(x, value) {
                return false;
            }
            for (newx, newvalue) in self.optimize() {
                queue.push_back(Substite(newx, newvalue));
            }
        }

        true
    }

    fn substitute(&mut self, x: usize, value: u8) -> bool {
        if let Some(existing) = self.substitutions.get(&x) {
            return value == *existing;
        }
        self.substitutions.insert(x, value);

        for eq in self.equations.iter_mut() {
            if !eq.substitute(x, value) {
                return false;
            }
        }
        true
    }

    fn optimize(&mut self) -> HashMap<usize, u8> {
        let mut possible_substitutions = HashMap::new();
        let mut eqs_to_drop = vec![];

        for (idx, eq) in self.equations.iter_mut().enumerate() {
            if eq.is_completed() {
                eqs_to_drop.push(idx);
            } else if let Some((x, value)) = eq.optimize() {
                possible_substitutions.insert(x, value);
            }
        }
        for idx in eqs_to_drop.into_iter().rev() {
            self.equations.remove(idx);
        }

        possible_substitutions
    }

    fn xs(&self) -> HashMap<usize, u8> {
        self.substitutions.clone()
    }

    fn all_solutions(self) -> Vec<HashMap<usize, u8>> {
        let Some((x, range)) = self.best_assumption() else {
            return vec![self.xs()];
        };

        let mut out = vec![];

        for value in range {
            let mut next = self.clone();
            if next.substitute_and_optimize(x, value) {
                let mut solutions = next.all_solutions();
                out.append(&mut solutions);
            }
        }

        out
    }

    fn min_solution(self) -> Option<u64> {
        self.all_solutions()
            .into_iter()
            .map(|xs| xs.into_values().map(|e| e as u64).sum::<u64>())
            .min()
    }
}

impl std::fmt::Display for EqSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = vec![];
        for eq in &self.equations {
            buf.push(format!("{eq}"));
        }
        for (x, value) in &self.substitutions {
            buf.push(format!("x{x}={value}"));
        }
        write!(f, "{}", buf.join(" | "))
    }
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 33)
}
