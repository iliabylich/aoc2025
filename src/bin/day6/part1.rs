fn main() {
    println!("{}", solve(include_str!("input1")))
}

#[derive(Debug)]
enum Op {
    Plus,
    Mul,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Op::Plus,
            "*" => Op::Mul,
            _ => panic!("unsupported op {s}"),
        }
    }

    fn reduce(&self, numbers: &[u64]) -> u64 {
        let mut out: u64 = match self {
            Self::Mul => 1,
            Self::Plus => 0,
        };

        for num in numbers {
            match self {
                Op::Plus => out = out.checked_add(*num).unwrap(),
                Op::Mul => out = out.checked_mul(*num).unwrap(),
            }
        }

        out
    }
}

fn solve(input: &str) -> u64 {
    let data = parse(input);
    let mut out = 0;

    for idx in 0..data[0].len() {
        let numbers = data[0..data.len() - 1]
            .iter()
            .map(|row| row[idx])
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let op = Op::parse(data[data.len() - 1][idx]);
        let result = op.reduce(&numbers);

        out += result;
    }

    out
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 4277556)
}
