use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        return Pos { x: x, y: y };
    }
}
fn move_head(dir: &str, head_pos: &Pos) -> Pos {
    return match dir {
        "R" => Pos::new(head_pos.x + 1, head_pos.y),
        "L" => Pos::new(head_pos.x - 1, head_pos.y),
        "U" => Pos::new(head_pos.x, head_pos.y + 1),
        "D" => Pos::new(head_pos.x, head_pos.y - 1),
        _ => panic!(),
    };
}
fn mv(a: i32, b: i32) -> i32 {
    return if a.abs() > 1 {
        a - a.signum()
    } else if a.abs() == 1 && b.abs() > 1 {
        a
    } else {
        0
    };
}
fn step(head_pos: &Pos, tail_pos: &Pos) -> Pos {
    let dx = head_pos.x - tail_pos.x;
    let dy = head_pos.y - tail_pos.y;

    return Pos::new(tail_pos.x + mv(dx, dy), tail_pos.y + mv(dy, dx));
}

fn func(lines: impl Iterator<Item = String>, length: usize) {
    let mut snake = vec![Pos::new(0, 0); length];
    let mut positions = HashSet::new();
    for line in lines {
        let (dir, dist) = line.split_whitespace().next_tuple().unwrap();
        println!("{dir} {dist}");
        for _ in 0..dist.parse().unwrap() {
            snake[length - 1] = move_head(dir, &snake[length - 1]);

            for pos in (1..length).rev() {
                snake[pos - 1] = step(&snake[pos], &snake[pos - 1]);
                println!("H ({pos}): {:?} ", snake[pos]);
            }
            positions.insert(snake[0].clone());
        }
    }

    println!("Num positions: {}", positions.len());
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 2);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, 10);
}
