use itertools::iproduct;
use regex::Regex;
use std::{cmp::max, collections::HashMap};

#[derive(Clone, Debug)]
struct Node {
    name: String,
    flow: i32,
    idx: usize,
}

impl Node {
    fn new(name: String, flow: i32) -> Node {
        Node {
            name: name,
            flow: flow,
            idx: 0,
        }
    }
}
#[derive(Debug)]
struct Edge {
    src: usize,
    dest: usize,
    cost: i32,
}

impl Edge {}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Vec<Edge>>,
    names: HashMap<String, usize>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            names: HashMap::new(),
        }
    }

    fn add_node(&mut self, n: Node) {
        let name = n.name.to_owned();
        self.nodes.push(n);
        self.edges.push(Vec::new());
        let idx = self.nodes.len() - 1;
        self.names.insert(name, idx);
    }

    fn add_edge(&mut self, n1: usize, n2: usize) {
        self.edges[n1].push(Edge {
            src: n1,
            dest: n2,
            cost: 1,
        });
    }

    fn node(&self, name: &str) -> &Node {
        &self.nodes[*self.names.get(name).unwrap()]
    }

    fn find(&self, name: &str) -> usize {
        *self.names.get(name).unwrap()
    }

    fn node_mut(&mut self, name: &str) -> &mut Node {
        &mut self.nodes[*self.names.get(name).unwrap()]
    }

    fn edges(&self, n: usize) -> &Vec<Edge> {
        &self.edges[n]
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

fn score(
    graph: &Graph,
    dists: &Vec<Vec<i32>>,
    valves_to_open: &Vec<usize>,
    state: u64,
    start: usize,
    current_time: i32,
    total_time: i32,
    state_scores: &mut HashMap<u64, i32>,
    flow_so_far: i32,
) {
    let remaining_time = total_time - current_time;

    for valve in valves_to_open {
        let dist = dists[start][*valve];

        if dist >= remaining_time {
            continue;
        }

        let minutes_left = remaining_time - dist - 1;
        let flow = graph.nodes[*valve].flow * minutes_left;
        let new_state = state | 1 << *valve;
        let state_entry = state_scores.entry(new_state).or_insert(0);
        *state_entry = max(*state_entry, flow_so_far + flow);

        let next_to_open = valves_to_open.clone().into_iter().filter(|x| x != valve);
        score(
            graph,
            dists,
            &next_to_open.collect(),
            new_state,
            *valve,
            current_time + dist + 1,
            total_time,
            state_scores,
            flow_so_far + flow,
        );
    }
}

fn func(lines: impl Iterator<Item = String>, max_time: i32, part1: bool) {
    let mut graph = Graph::new();

    let mut foo: HashMap<String, Vec<String>> = HashMap::new();

    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9-]+); tunnels? leads? to valves? (.*)")
        .unwrap();
    for line in lines {
        let caps = re
            .captures(line.as_str())
            .unwrap()
            .iter()
            .map(|c| c.unwrap().as_str())
            .collect::<Vec<&str>>();
        let id = caps[1];
        let flow: i32 = caps[2].parse().unwrap();
        let dests: Vec<String> = caps[3].split(", ").map(|d| d.to_owned()).collect();

        println!("{id} {flow} {:?}", dests);
        let node = Node::new(id.to_owned(), flow);
        graph.add_node(node);
        foo.insert(id.to_owned(), dests);
    }

    for (src, dests) in foo {
        for dest in dests {
            let a = graph.find(&src);
            let b = graph.find(&dest);
            graph.add_edge(a, b);
        }
    }

    // Calculate all pairs shortest paths (Floyd-Warshall)
    let num = graph.node_count();
    let mut dists: Vec<Vec<i32>> = vec![vec![10000; num]; num];
    for e in graph.edges.iter().flatten() {
        dists[e.src][e.dest] = 1;
        dists[e.dest][e.src] = 1;
    }
    for n in 0..num {
        dists[n][n] = 0;
    }
    for k in 0..num {
        for i in 0..num {
            for j in 0..num {
                if dists[i][j] > dists[i][k] + dists[k][j] {
                    dists[i][j] = dists[i][k] + dists[k][j];
                }
            }
        }
    }
    // Only consider valves with non-zero flow
    let valves_to_open = (0..num)
        .filter(|n| graph.nodes[*n].flow > 0)
        .collect::<Vec<usize>>();

    // Calculate max score for each combination of open valves
    let mut state_scores = HashMap::new();
    score(
        &graph,
        &dists,
        &valves_to_open,
        0,
        graph.find("AA"),
        0,
        max_time,
        &mut state_scores,
        0,
    );

    let score = if part1 {
        // Part 1: Return max of seen scores
        *state_scores.values().max().unwrap_or(&0)
    } else {
        // Part 2: Return max of sum of two non-overlapping sets of open valves
        iproduct!(state_scores.iter(), state_scores.iter())
            .filter(|(a, b)| *a.0 & *b.0 == 0)
            .map(|(a, b)| *a.1 + *b.1)
            .max()
            .unwrap_or(0)
    };

    println!("Score: {score}");
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 30, true);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, 26, false);
}
