use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let graph = Graph::parse(input);

    let mut f = NumberOfWays::new();

    for (from, bucket) in graph.edges.iter() {
        for to in bucket {
            f.add_edge(*from, *to);
        }
    }

    let one = f.get(Node::SVR, Node::DAC, &graph)
        * f.get(Node::DAC, Node::FFT, &graph)
        * f.get(Node::FFT, Node::OUT, &graph);

    let two = f.get(Node::SVR, Node::FFT, &graph)
        * f.get(Node::FFT, Node::DAC, &graph)
        * f.get(Node::DAC, Node::OUT, &graph);

    one + two
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node([u8; 3]);
impl Node {
    const SVR: Self = Self([b's', b'v', b'r']);
    const OUT: Self = Self([b'o', b'u', b't']);
    const DAC: Self = Self([b'd', b'a', b'c']);
    const FFT: Self = Self([b'f', b'f', b't']);

    fn new(s: &str) -> Self {
        assert_eq!(s.len(), 3);
        let bytes: [u8; 3] = s.as_bytes().try_into().unwrap();
        Self(bytes)
    }
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Debug)]
struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn parse(input: &str) -> Self {
        let mut nodes: HashSet<Node> = HashSet::new();
        let mut edges: HashMap<Node, Vec<Node>> = HashMap::new();

        for line in input.lines() {
            let (node, connected) = line.split_once(": ").unwrap();
            let node = Node::new(node);
            nodes.insert(node);
            for connected in connected.split(' ') {
                let connected = Node::new(connected);
                nodes.insert(connected);
                edges.entry(node).or_default().push(connected);
            }
        }

        Graph {
            nodes: nodes.into_iter().collect(),
            edges,
        }
    }
}

#[derive(Debug)]
struct NumberOfWays {
    map: HashMap<Node, HashMap<Node, u64>>,
}
impl NumberOfWays {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        *self.map.entry(from).or_default().entry(to).or_default() = 1;
    }

    fn get(&mut self, from: Node, to: Node, graph: &Graph) -> u64 {
        if let Some(bucket) = self.map.get(&from) {
            if let Some(num) = bucket.get(&to) {
                return *num;
            }
        }

        let Some(edges) = graph.edges.get(&from) else {
            *self.map.entry(from).or_default().entry(to).or_default() = 0;
            return 0;
        };

        // f(a, b) = f(a, x) * f(x, b) WHERE a and x are connected
        let mut total = 0;
        for x in edges.iter().copied() {
            if from != x && x != to {
                total += self.get(from, x, graph) * self.get(x, to, graph);
            }
        }

        *self.map.entry(from).or_default().entry(to).or_default() = total;
        total
    }
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input2")), 2)
}
