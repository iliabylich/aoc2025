use std::ops::RangeInclusive;

fn main() {
    println!("{}", solve(include_str!("input1")))
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn as_range(&self) -> RangeInclusive<u64> {
        self.start..=self.end
    }

    fn overlap(&self, other: &Self) -> bool {
        let this = self.as_range();
        this.contains(&other.start) || this.contains(&other.end)
    }

    fn len(&self) -> u64 {
        if self.start == 0 && self.end == 0 {
            return 0;
        }
        self.end - self.start + 1
    }
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
        ranges.push(Range { start, end });
    }

    ranges.sort_unstable_by_key(|range| range.start);

    let mut left_idx = 0;
    let mut right_idx = 1;

    loop {
        let Some(left) = ranges.get(left_idx).copied() else {
            left_idx = right_idx;
            right_idx += 1;
            continue;
        };
        let Some(right) = ranges.get(right_idx) else {
            break;
        };

        if left.overlap(right) {
            ranges[left_idx].end = std::cmp::max(ranges[left_idx].end, ranges[right_idx].end);
            ranges[right_idx] = Range { start: 0, end: 0 };
            right_idx += 1;
        } else {
            left_idx = right_idx;
            right_idx += 1;
        }
    }

    ranges.into_iter().map(|range| range.len()).sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 14)
}
