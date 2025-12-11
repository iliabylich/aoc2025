use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", solve(include_str!("input1")))
}

fn solve(input: &str) -> u64 {
    let graph = Graph::parse(input);

    let mut out = HashSet::new();
    let mut path = vec![Node::new("you")];
    recurse(&graph, &mut path, &mut out);

    out.len() as u64
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node([u8; 3]);
impl Node {
    const OUT: Self = Self([b'o', b'u', b't']);

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
        let mut nodes: Vec<Node> = vec![];
        let mut edges: HashMap<Node, Vec<Node>> = HashMap::new();

        for line in input.lines() {
            let (node, connected) = line.split_once(": ").unwrap();
            let node = Node::new(node);
            nodes.push(node);
            for connected in connected.split(' ') {
                let connected = Node::new(connected);
                edges.entry(node).or_default().push(connected);
            }
        }

        Graph { nodes, edges }
    }
}

fn recurse(graph: &Graph, path: &mut Vec<Node>, out: &mut HashSet<Vec<Node>>) {
    let last = *path.last().unwrap();

    if last == Node::OUT {
        out.insert(path.clone());
        return;
    }

    let Some(connected) = graph.edges.get(&last) else {
        return;
    };

    for next in connected {
        assert!(!path.contains(next), "{path:?} {next:?}");
        path.push(*next);
        recurse(graph, path, out);
        path.pop();
    }
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input0")), 5)
}
