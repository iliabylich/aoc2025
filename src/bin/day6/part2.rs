fn main() {
    println!("{}", solve(include_str!("input1")))
}

#[derive(Debug)]
enum Op {
    Plus(usize),
    Mul(usize),
}

impl Op {
    fn len(&self) -> usize {
        match self {
            Self::Mul(len) | Self::Plus(len) => *len,
        }
    }

    fn reduce(&self, numbers: &[u64]) -> u64 {
        let mut out: u64 = match self {
            Self::Mul(_) => 1,
            Self::Plus(_) => 0,
        };

        for num in numbers {
            match self {
                Self::Plus(_) => out = out.checked_add(*num).unwrap(),
                Self::Mul(_) => out = out.checked_mul(*num).unwrap(),
            }
        }

        out
    }
}

fn combine_nums(matrix: &[&[u8]]) -> Vec<u64> {
    let mut out = vec![];

    let numrows = matrix.len();
    let numcols = matrix[0].len();
    for col in 0..numcols {
        let mut num: u64 = 0;
        for row in 0..numrows {
            let cell = *matrix.get(row).unwrap().get(col).unwrap();
            let digit = match cell {
                b' ' => continue,
                _ => cell - b'0',
            };
            num = num * 10 + digit as u64;
        }
        out.push(num);
    }
    out
}

fn solve(input: &str) -> u64 {
    let (ops, rows) = parse(input.as_bytes());
    let mut out = 0;

    for (idx, op) in ops.iter().enumerate() {
        let nums = rows.iter().map(|row| row[idx]).collect::<Vec<_>>();

        let combined = combine_nums(&nums);

        let result = op.reduce(&combined);
        out += result;
    }

    out
}

fn parse(input: &[u8]) -> (Vec<Op>, Vec<Vec<&[u8]>>) {
    let mut lines = input.split(|b| *b == b'\n').collect::<Vec<_>>();
    lines.pop();

    let ops = parse_ops(lines.pop().unwrap());

    let rows = parse_rows(lines, &ops);

    for (idx, op) in ops.iter().enumerate() {
        for row in &rows {
            let num = row.get(idx).unwrap();
            assert_eq!(num.len(), op.len());
        }
    }

    (ops, rows)
}

fn parse_ops(bytes: &[u8]) -> Vec<Op> {
    let mut ops = vec![];
    assert!(matches!(bytes[0], b'+' | b'*'));

    let mut start_idx = 0;
    let mut end_idx = 1;
    while end_idx < bytes.len() {
        while let Some(b' ') = bytes.get(end_idx) {
            end_idx += 1;
        }

        let op = &bytes[start_idx..end_idx];

        let sign = op[0];
        let len = op.len() - 1;

        let op = match sign {
            b'+' => Op::Plus(len),
            b'*' => Op::Mul(len),
            _ => panic!("wrong op {}", sign as char),
        };

        ops.push(op);

        start_idx = end_idx;
        end_idx = start_idx + 1;
    }

    match ops.last_mut().unwrap() {
        Op::Plus(len) | Op::Mul(len) => *len += 1,
    }

    ops
}

fn parse_rows<'a>(lines: Vec<&'a [u8]>, ops: &[Op]) -> Vec<Vec<&'a [u8]>> {
    let mut out = vec![];

    for mut line in lines {
        let mut parsed = vec![];
        for op in ops {
            let (left, right) = line.split_at_checked(op.len()).unwrap();
            // println!(
            //     "{:?} {:?}",
            //     std::str::from_utf8(left).unwrap(),
            //     std::str::from_utf8(right).unwrap()
            // );
            parsed.push(left);

            if right.is_empty() {
                line = &[];
            } else {
                line = &right[1..];
            }
        }
        out.push(parsed);
    }
    out
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 3263827)
}
