fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let points = parse(input);
    let mut max = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let w = p1.0.abs_diff(p2.0) + 1;
            let h = p1.1.abs_diff(p2.1) + 1;

            let area = w * h;

            max = std::cmp::max(max, area);
        }
    }

    max
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let (row, col) = line.split_once(',').unwrap();
            (row.parse().unwrap(), col.parse().unwrap())
        })
        .collect()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 50)
}
