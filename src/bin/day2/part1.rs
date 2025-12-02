fn main() {
    let input = include_str!("input1");

    println!("{}", solve(input))
}

fn solve(input: &str) -> u64 {
    let input = input.strip_suffix('\n').unwrap();
    let mut out = 0;

    for part in input.split(',') {
        let (first, last) = part.split_once('-').unwrap();
        let first = first.parse::<u64>().unwrap();
        let last = last.parse::<u64>().unwrap();
        out += process_range(first, last);
    }

    out
}

fn process_range(first: u64, last: u64) -> u64 {
    let mut sum = 0_u64;
    for i in first..=last {
        if !is_valid(i) {
            sum = sum.checked_add(i).unwrap();
        }
    }
    sum
}

fn is_valid(n: u64) -> bool {
    let ndigits = numdigits(n);
    if ndigits % 2 == 1 {
        return true;
    }

    let mod_ = 10_u64.pow(ndigits / 2);
    let left = n / mod_;
    let right = n % mod_;
    left != right
}
#[test]
fn test_is_valid() {
    assert!(is_valid(1));
    assert!(is_valid(9));

    assert!(is_valid(12));
    assert!(!is_valid(11));
    assert!(!is_valid(33));

    assert!(is_valid(123));
    assert!(is_valid(111));

    assert!(is_valid(1234));
    assert!(!is_valid(1212));
    assert!(!is_valid(4545));
}

fn numdigits(mut n: u64) -> u32 {
    let mut out = 0;
    while n > 0 {
        out += 1;
        n /= 10;
    }
    out
}
#[test]
fn test_numdigits() {
    assert_eq!(numdigits(1), 1);
    assert_eq!(numdigits(11), 2);
    assert_eq!(numdigits(1111), 4);
}

#[test]
fn test() {
    let input = include_str!("input0");
    let output = solve(input);
    assert_eq!(output, 1227775554);
}
