fn main() {
    let input = include_str!("input1");

    println!("{}", solve(input))
}

fn solve(input: &str) -> u32 {
    let mut acc = 50;
    let mut out = 0;

    for line in input.lines() {
        // let line = line.as_bytes();
        let mul = match line.as_bytes()[0] {
            b'L' => -1,
            b'R' => 1,
            _ => panic!("wrong direction"),
        };
        let num: i32 = line[1..].parse::<i32>().unwrap() * mul;

        let mut prev = acc + mul;
        acc += num;

        loop {
            if prev % 100 == 0 {
                out += 1;
            }
            prev += mul;
            if prev == acc + mul {
                break;
            }
        }
    }

    out
}

#[test]
fn test() {
    let input = include_str!("input0");
    let output = solve(input);
    assert_eq!(output, 6);
}
