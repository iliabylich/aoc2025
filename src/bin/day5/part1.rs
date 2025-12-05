fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let mut ranges = vec![];

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ranges.push(start..=end);
    }

    let mut out = 0;
    for line in lines {
        if line.is_empty() {
            break;
        }
        let id: u64 = line.parse().unwrap();

        let fresh = ranges.iter().any(|range| range.contains(&id));

        if fresh {
            out += 1;
        }
    }

    out
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 3)
}
