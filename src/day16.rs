use regex::Regex;
use std::collections::HashMap;

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

fn score2(
    graph: &Graph,
    dists: &Vec<Vec<i32>>,
    valves_to_open: &Vec<usize>,
    path: &Vec<usize>,
    already_opened: &Vec<usize>,
    start: usize,
    current_time: i32,
    total_time: i32,
) -> (i32, Vec<usize>) {
    let remaining_time = total_time - current_time;

    let mut candidates: Vec<(i32, Vec<usize>)> = Vec::new();
    for valve in valves_to_open {
        let dist = dists[start][*valve];

        if dist >= remaining_time || already_opened.contains(valve) {
            continue;
        }

        let minutes_left = remaining_time - dist - 1;
        let flow = graph.nodes[*valve].flow * minutes_left;

        let next_to_open: Vec<usize> = valves_to_open
            .iter()
            .filter(|v| *v != valve)
            .map(|v| *v)
            .collect();

        let mut next_path = path.clone();
        next_path.push(*valve);

        let candidate = score2(
            graph,
            dists,
            &next_to_open,
            &next_path,
            &already_opened,
            *valve,
            current_time + dist + 1,
            total_time,
        );

        let full_flow = flow + candidate.0;
        let mut full_path = path.clone();
        full_path.extend(candidate.1);
        candidates.push((full_flow, full_path));
    }

    let mut flow: (i32, Vec<usize>) = (0, Vec::new());
    for c in candidates {
        if c.0 > flow.0 {
            flow = c;
        }
    }
    flow
}

fn func(lines: impl Iterator<Item = String>) {
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

    // all pairs shortest paths (Floyd-Warshall)
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

    for i in 0..num {
        for j in 0..num {
            print!("{:2} ", dists[i][j]);
        }
        println!();
    }

    let valves_to_open = (0..num)
        .filter(|n| graph.nodes[*n].flow > 0)
        .collect::<Vec<usize>>();
    dbg!(&valves_to_open);
    //let s = score(&graph, &HashSet::new(), "AA", 0, &mut HashMap::new());
    let s = score2(
        &graph,
        &dists,
        &valves_to_open,
        &vec![],
        &Vec::new(),
        graph.find("AA"),
        0,
        26,
    );

    dbg!(&s);
    let s2 = score2(
        &graph,
        &dists,
        &valves_to_open,
        &vec![graph.find("AA")],
        &s.1,
        graph.find("AA"),
        0,
        26,
    );
    dbg!(&s2);
    println!("Score: {}", s.0 + s2.0);
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines);
}

pub fn part2(_lines: impl Iterator<Item = String>) {}
