use std::collections::HashMap;

fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let mut out = 0;

    for line in input.lines() {
        let line = line.as_bytes();
        let line = line.iter().copied().map(|b| b - b'0').collect::<Vec<_>>();

        out += process_line(&line);
    }

    out
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Num {
    digits: [u8; 12],
}

impl Num {
    fn to_u64(&self) -> u64 {
        let mut out = 0;
        for digit in self.digits {
            out = out * 10 + digit as u64;
        }
        out
    }

    fn new<const N: usize>(digits: [u8; N]) -> Self {
        let mut out = [0; 12];
        for (idx, digit) in digits.into_iter().enumerate() {
            out[idx] = digit;
        }
        Num { digits: out }
    }
}

fn process_line(line: &[u8]) -> u64 {
    let mut max = Num { digits: [0; 12] };

    let mut cache = HashMap::new();

    for d1pos in find(line, 0, &mut cache) {
        let d1 = line[d1pos];
        if Num::new([d1]) < max {
            continue;
        }

        for d2pos in find(line, d1pos + 1, &mut cache) {
            let d2 = line[d2pos];
            if Num::new([d1, d2]) < max {
                continue;
            }

            for d3pos in find(line, d2pos + 1, &mut cache) {
                let d3 = line[d3pos];
                if Num::new([d1, d2, d3]) < max {
                    continue;
                }

                for d4pos in find(line, d3pos + 1, &mut cache) {
                    let d4 = line[d4pos];
                    if Num::new([d1, d2, d3, d4]) < max {
                        continue;
                    }

                    for d5pos in find(line, d4pos + 1, &mut cache) {
                        let d5 = line[d5pos];
                        if Num::new([d1, d2, d3, d4, d5]) < max {
                            continue;
                        }

                        for d6pos in find(line, d5pos + 1, &mut cache) {
                            let d6 = line[d6pos];
                            if Num::new([d1, d2, d3, d4, d5, d6]) < max {
                                continue;
                            }

                            for d7pos in find(line, d6pos + 1, &mut cache) {
                                let d7 = line[d7pos];

                                for d8pos in find(line, d7pos + 1, &mut cache) {
                                    let d8 = line[d8pos];

                                    for d9pos in find(line, d8pos + 1, &mut cache) {
                                        let d9 = line[d9pos];

                                        for d10pos in find(line, d9pos + 1, &mut cache) {
                                            let d10 = line[d10pos];

                                            for d11pos in find(line, d10pos + 1, &mut cache) {
                                                let d11 = line[d11pos];

                                                for d12pos in find(line, d11pos + 1, &mut cache) {
                                                    let d12 = line[d12pos];

                                                    let candidate = Num {
                                                        digits: [
                                                            d1, d2, d3, d4, d5, d6, d7, d8, d9,
                                                            d10, d11, d12,
                                                        ],
                                                    };

                                                    // println!("{candidate:?}");

                                                    max = std::cmp::max(max, candidate);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", max);

    max.to_u64()
}

fn find(line: &[u8], starting_from: usize, cache: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if let Some(cached) = cache.get(&starting_from) {
        return cached.clone();
    }
    let value = find0(&line[starting_from..], starting_from);
    cache.insert(starting_from, value.clone());
    value
}

fn find0(bytes: &[u8], offset: usize) -> Vec<usize> {
    let mut out = vec![];

    for n in (1..=9).rev() {
        if let Some(pos) = bytes.iter().position(|e| *e == n) {
            out.push(pos + offset);
        }
    }

    out
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 3121910778619)
}

// 171039099596062
