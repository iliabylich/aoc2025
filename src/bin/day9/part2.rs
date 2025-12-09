use std::{
    collections::HashMap,
    ops::Add,
    sync::atomic::{AtomicU64, Ordering},
};

fn main() {
    println!("{}", solve(include_str!("input1")))
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vec2(i64, i64);

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Line {
    H(HLine),
    V(VLine),
}
impl Line {
    fn new(start: Vec2, end: Vec2) -> Self {
        if start.0 == end.0 {
            Self::H(HLine {
                row: start.0,
                start: start.1,
                end: end.1,
                start_goes_inside: None,
                end_goes_inside: None,
            })
        } else if start.1 == end.1 {
            Self::V(VLine {
                col: start.1,
                start: start.0,
                end: end.0,
                start_goes_inside: None,
                end_goes_inside: None,
            })
        } else {
            panic!("neither H to V")
        }
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct HLine {
    row: i64,
    start: i64,
    end: i64,
    start_goes_inside: Option<bool>,
    end_goes_inside: Option<bool>,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct VLine {
    col: i64,
    start: i64,
    end: i64,
    start_goes_inside: Option<bool>,
    end_goes_inside: Option<bool>,
}

fn render_svg(path: &str, dots: &[Vec2]) {
    let mut svg =
        String::from("<svg viewBox=\"0 0 100000 100000\" xmlns=\"http://www.w3.org/2000/svg\">");

    for i in 0..dots.len() {
        let mut j = i + 1;
        if j == dots.len() {
            j = 0;
        }
        let Vec2(x1, y1) = dots[i];
        let Vec2(x2, y2) = dots[j];
        svg += &format!(
            "\n<line x1=\"{y1}\" y1=\"{x1}\" x2=\"{y2}\" y2=\"{x2}\" stroke=\"red\" stroke-width=\"100px\"  />",
        );
    }

    svg += "</svg>";

    std::fs::write(path, svg).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    start: i64,
    end: i64,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Bucket {
    items: Vec<Item>,
}
impl Bucket {
    fn compact(&mut self) {
        loop {
            let mut made_progress = false;

            self.items.sort_unstable();

            for idx in 0..self.items.len() {
                let left = self.items[idx];
                let Some(right) = self.items.get(idx + 1).copied() else {
                    break;
                };

                if left.end == right.start {
                    self.items[idx] = Item {
                        start: left.start,
                        end: right.end,
                    };
                    self.items.swap_remove(idx + 1);
                    made_progress = true;
                    break;
                }
            }

            if !made_progress {
                break;
            }
        }
    }

    fn push(&mut self, new: Item) {
        self.items.push(new);
        self.compact();
    }

    fn expand(&mut self) {
        if self.items.len() == 1 {
            return;
        }

        let mut new = vec![];
        for (idx, window) in self.items.windows(2).enumerate() {
            if idx % 2 == 1 {
                continue;
            }
            let [prev, next] = window.try_into().unwrap();
            new.push(Item {
                start: prev.end,
                end: next.start,
            });
        }

        self.items.append(&mut new);
        self.compact();
    }
}

fn solve(input: &str) -> u64 {
    let dots = parse(input);
    render_svg("full.svg", &dots);

    let mut shape = Vec::new();

    for i in 0..dots.len() {
        let start = dots[i];
        let mut j = i + 1;
        if j == dots.len() {
            j = 0;
        }
        let end = dots[j];

        let line = Line::new(start, end);
        shape.push(line);
    }

    let mut points_grouped_by_row = HashMap::<i64, Bucket>::new();

    for line in shape.iter() {
        match line {
            Line::H(h) => {
                let min = std::cmp::min(h.start, h.end);
                let max = std::cmp::max(h.start, h.end);
                points_grouped_by_row.entry(h.row).or_default().push(Item {
                    start: min,
                    end: max,
                });
            }
            Line::V(v) => {
                let min = std::cmp::min(v.start, v.end);
                let max = std::cmp::max(v.start, v.end);
                for row in min..=max {
                    points_grouped_by_row.entry(row).or_default().push(Item {
                        start: v.col,
                        end: v.col,
                    });
                }
            }
        }
    }

    for bucket in points_grouped_by_row.values_mut() {
        bucket.expand();
    }

    let is_inside = |dot: Vec2| -> bool {
        let Some(bucket) = points_grouped_by_row.get(&dot.0) else {
            return false;
        };

        for item in &bucket.items {
            let fits = (item.start..=item.end).contains(&dot.1);
            if fits {
                return true;
            }
        }
        false
    };

    let check = |d1: Vec2, d2: Vec2, max: u64| -> Option<u64> {
        let w = d1.0.abs_diff(d2.0) + 1;
        let h = d1.1.abs_diff(d2.1) + 1;
        let area = w * h;

        if area < max {
            return None;
        }

        let top = std::cmp::min(d1.0, d2.0);
        let bottom = std::cmp::max(d1.0, d2.0);
        let left = std::cmp::min(d1.1, d2.1);
        let right = std::cmp::max(d1.1, d2.1);

        // top side
        let row = top;
        for col in left..=right {
            if !is_inside(Vec2(row, col)) {
                return None;
            }
        }

        // bottom side
        let row = bottom;
        for col in left..=right {
            if !is_inside(Vec2(row, col)) {
                return None;
            }
        }

        // left side
        let col = left;
        for row in top..=bottom {
            if !is_inside(Vec2(row, col)) {
                return None;
            }
        }

        // right side
        let col = right;
        for row in top..=bottom {
            if !is_inside(Vec2(row, col)) {
                return None;
            }
        }

        Some(area)
    };

    let mut combinations = vec![];
    for i in 0..dots.len() {
        for j in (i + 1)..dots.len() {
            combinations.push((dots[i], dots[j]));
        }
    }

    let max = AtomicU64::new(0);

    let chunk_size = combinations.len() / 10;
    let mut chunks = vec![];
    let mut slice = combinations.as_mut_slice();
    while !slice.is_empty() {
        let (chunk, rem) = slice.split_at_mut(chunk_size);
        chunks.push(chunk);
        slice = rem;
    }

    std::thread::scope(|scope| {
        let mut threads = vec![];

        for chunk in chunks {
            let thread = scope.spawn(|| {
                for (d1, d2) in chunk {
                    if let Some(newmax) = check(*d1, *d2, max.load(Ordering::Relaxed)) {
                        max.fetch_max(newmax, Ordering::Relaxed);
                    }
                }
            });
            threads.push(thread);
        }

        for t in threads {
            t.join().unwrap();
        }
    });

    max.load(Ordering::Relaxed)
}

fn parse(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .map(|line| {
            let (col, row) = line.split_once(',').unwrap();
            Vec2(row.parse().unwrap(), col.parse().unwrap())
        })
        .collect()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 24)
}
