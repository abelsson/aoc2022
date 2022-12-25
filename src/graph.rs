use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub flow: i32,
    pub idx: usize,
}

impl Node {
    pub fn new(name: String, flow: i32) -> Node {
        Node {
            name: name,
            flow: flow,
            idx: 0,
        }
    }
}
#[derive(Debug)]
pub struct Edge {
    pub src: usize,
    pub dest: usize,
    pub cost: i32,
}

impl Edge {}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<Edge>>,
    pub names: HashMap<String, usize>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            names: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, n: Node) {
        let name = n.name.to_owned();
        self.nodes.push(n);
        self.edges.push(Vec::new());
        let idx = self.nodes.len() - 1;
        self.names.insert(name, idx);
    }

    pub fn add_edge(&mut self, n1: usize, n2: usize) {
        self.edges[n1].push(Edge {
            src: n1,
            dest: n2,
            cost: 1,
        });
    }

    pub fn node(&self, name: &str) -> &Node {
        &self.nodes[*self.names.get(name).unwrap()]
    }

    pub fn find(&self, name: &str) -> usize {
        *self.names.get(name).unwrap()
    }

    pub fn node_mut(&mut self, name: &str) -> &mut Node {
        &mut self.nodes[*self.names.get(name).unwrap()]
    }

    pub fn edges(&self, n: usize) -> &Vec<Edge> {
        &self.edges[n]
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
