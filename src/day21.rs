use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Node {
    value: Option<i64>,
    left_idx: usize,
    op: char,
    right_idx: usize,
}

fn evaluate(nodes: &mut Vec<Node>, idx: usize) -> i64 {
    let n = *nodes.get(idx).unwrap();
    if let Some(value) = n.value {
        return value;
    }

    let left = evaluate(nodes, n.left_idx);
    let right = evaluate(nodes, n.right_idx);

    let res = match n.op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => panic!(),
    };

    nodes.get_mut(idx).unwrap().value = Some(res);
    return res;
}
fn func(lines_it: impl Iterator<Item = String>, part2: bool) {
    let lines = lines_it.collect_vec();
    let mut nodes: Vec<Node> = Vec::new();
    let mut name_map: HashMap<&str, usize> = HashMap::new();
    for line in lines.iter() {
        let name = line.split(":").take(1).collect_vec()[0];
        if part2 && name == "humn" {
            nodes.push(Node {
                value: Some(3_759_566_892_642),
                //value: Some(301),
                left_idx: 0,
                op: '?',
                right_idx: 0,
            });
        } else {
            nodes.push(Node {
                value: None,
                left_idx: 0,
                op: '?',
                right_idx: 0,
            });
        }
        name_map.insert(name, nodes.len() - 1);
    }
    for line in lines.iter() {
        let parts = line.split_whitespace().collect_vec();
        let name = parts[0].trim_end_matches(':');
        let value = &mut nodes[*name_map.get(&name).unwrap()];
        if parts.len() == 2 && value.value.is_none() {
            value.value = Some(parts[1].parse().unwrap());
        }
        if parts.len() == 4 {
            let left_idx = *name_map.get(parts[1]).unwrap();
            let right_idx = *name_map.get(parts[3]).unwrap();
            let op = parts[2].chars().take(1).collect_vec()[0];
            value.left_idx = left_idx;
            value.right_idx = right_idx;
            value.op = op;
        }
    }

    dbg!(&nodes);

    if !part2 {
        let root = *name_map.get("root").unwrap();
        let res = evaluate(&mut nodes, root);
        println!("{res}");
    } else {
        let root = *name_map.get("root").unwrap();
        let left = nodes[root].left_idx;
        let right = nodes[root].right_idx;

        let r1 = evaluate(&mut nodes, left);

        let r2 = evaluate(&mut nodes, right);
        println!("{r1} == {r2}: diff {}", r1 - r2);
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, false);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, true);
}
