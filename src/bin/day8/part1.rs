use std::collections::{BinaryHeap, HashSet};

fn main() {
    println!("{}", solve(include_str!("input1"), 1000))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn parse(line: &str) -> Self {
        let (x, rest) = line.split_once(',').unwrap();
        let (y, z) = rest.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
}

fn distance(p1: Point, p2: Point) -> f64 {
    ((p1.x as f64 - p2.x as f64) * (p1.x as f64 - p2.x as f64)
        + (p1.y as f64 - p2.y as f64) * (p1.y as f64 - p2.y as f64)
        + (p1.z as f64 - p2.z as f64) * (p1.z as f64 - p2.z as f64))
        .sqrt()
}

#[derive(Debug)]
struct DistanceWithData<T> {
    d: f64,
    data: T,
}
impl<T> PartialEq for DistanceWithData<T> {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}
impl<T> Eq for DistanceWithData<T> {}
impl<T> PartialOrd for DistanceWithData<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.d.partial_cmp(&other.d)
    }
}
impl<T> Ord for DistanceWithData<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.d.partial_cmp(&other.d) {
            Some(ord) => ord,
            None => panic!("NAN"),
        }
    }
}

fn solve(input: &str, iterations: u64) -> u64 {
    let dots = input.lines().map(Point::parse).collect::<Vec<_>>();

    let mut distances = BinaryHeap::new();
    for p1_idx in 0..dots.len() {
        for p2_idx in (p1_idx + 1)..dots.len() {
            let p1 = dots[p1_idx];
            let p2 = dots[p2_idx];

            let d = distance(p1, p2);
            distances.push(DistanceWithData {
                d: -d,
                data: (p1_idx, p2_idx),
            });
        }
    }
    // println!("{distances:?}");

    let mut circuits = (0..dots.len())
        .map(|dot_idx| HashSet::from([dot_idx]))
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        // println!("\n\nIteration {iteration}");

        let Some(DistanceWithData {
            data: (p1_idx, p2_idx),
            ..
        }) = distances.pop()
        else {
            panic!("No more distances left");
        };

        // println!(
        //     "Dots are {:?} and {:?} (d = {})",
        //     dots[p1_idx], dots[p2_idx], d
        // );

        let p1_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains(&p1_idx))
            .unwrap();
        let p2_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains(&p2_idx))
            .unwrap();

        if p1_circuit_idx == p2_circuit_idx {
            continue;
        }

        let mut circuit1 = circuits.swap_remove(p1_circuit_idx);
        let circuit2 = circuits.swap_remove(
            circuits
                .iter()
                .position(|circuit| circuit.contains(&p2_idx))
                .unwrap(),
        );

        // println!("Merging {circuit1:?} and {circuit2:?}");

        circuit1.extend(circuit2);
        let merged = circuit1;

        // println!("Into {merged:?}");
        circuits.push(merged);
    }

    // println!("{circuits:?}");

    let mut sizes: Vec<u64> = circuits.into_iter().map(|s| s.len() as u64).collect();
    sizes.sort_unstable_by_key(|e| -(*e as i64));

    sizes[0] * sizes[1] * sizes[2]
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0"), 10), 40)
}
