fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let mut out = 0;

    for line in input.lines() {
        let line = line.as_bytes();

        let mut max = 0_u8;
        for (max1pos, max1) in line.iter().copied().enumerate() {
            for max2 in line.iter().skip(max1pos + 1).copied() {
                let candidate = (max1 - b'0') * 10 + max2 - b'0';
                max = max.max(candidate);
            }
        }

        out += max as u64;
    }

    out
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 357)
}
