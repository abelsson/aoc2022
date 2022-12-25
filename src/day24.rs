use std::collections::{HashMap, HashSet, VecDeque};

use crate::canvas::Canvas;

fn bfs(
    map: &Vec<Canvas>,
    start: &(i32, i32, i32),
    target: &(i32, i32),
    max_time: i32,
) -> Option<i32> {
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut come_from: HashMap<(i32, i32, i32), (i32, i32, i32)> = HashMap::new();
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();

    queue.push_back(*start);

    while let Some(current) = queue.pop_front() {
        if current.0 == target.0 && current.1 == target.1 {
            println!("{}", current.2);

            let mut pos = *come_from
                .iter()
                .find(|&(k, _)| k.0 == target.0 && k.1 == target.1)
                .unwrap()
                .1;
            let mut h: Vec<(i32, i32, i32)> = Vec::new();
            while &pos != start {
                h.push(pos);
                pos = *come_from.get(&pos).unwrap();
            }
            println!("{:?}", h);

            for &(x, y, time) in h.iter().rev() {
                let mut c = map[time as usize].clone();
                let prev = c.get_pixel(x, y);
                c.set_pixel(x, y, 33);
                c.display(0, 0, c.width, c.height);
                println!("{time} {x} {y} Prev = {prev}");
                assert!(prev == 0);
            }
            return Some(current.2);
        }
        let time: i32 = current.2;
        let cands = map[time as usize + 1].neighbours4(current.0, current.1, 0);
        for cand_pos in cands
            .iter()
            .filter(|(x, y)| map[time as usize + 1].get_pixel(*x, *y) == 0)
        {
            let cand = (cand_pos.0, cand_pos.1, time + 1);
            if !visited.contains(&cand) && cand.2 < max_time {
                visited.insert(cand);
                queue.push_back(cand);
                come_from.insert(cand, current);
            }
        }
        let cand = (current.0, current.1, time + 1);
        if !visited.contains(&cand)
            && cand.2 < max_time
            && map[time as usize + 1].get_pixel(cand.0, cand.1) == 0
        {
            visited.insert(cand);
            queue.push_back(cand);
            come_from.insert(cand, current);
        }
    }

    None
}
fn func(lines: impl Iterator<Item = String>) {
    let offset = 0;
    let w = 122; // 7, 8, 122
    let h = 27; // 7, 6, 27
    let mut canvas = Canvas::new(w, h);
    let mut blizzards: Vec<(i32, i32, char)> = Vec::new();
    let mut fields: Vec<Canvas> = Vec::new();

    for (y, line) in lines.enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            match *ch as char {
                '.' => canvas.set_pixel(x as i32 + offset, y as i32 + offset, 0),
                '#' => canvas.set_pixel(x as i32 + offset, y as i32 + offset, 1),
                '>' => blizzards.push((x as i32, y as i32, '>')),
                '^' => blizzards.push((x as i32, y as i32, '^')),
                '<' => blizzards.push((x as i32, y as i32, '<')),
                'v' => blizzards.push((x as i32, y as i32, 'v')),
                _ => (),
            }
        }
    }

    for _ in 0..800 {
        let mut c = canvas.clone();

        // record current blizzard state
        for &(x, y, _) in &blizzards {
            c.set_pixel(x, y, 2);
        }
        c.display(0, 0, w, h);
        fields.push(c);

        // update state
        for (x, y, dir) in &mut blizzards {
            let (new_x, new_y) = match dir {
                '>' => (*x + 1, *y),
                '^' => (*x, *y - 1),
                '<' => (*x - 1, *y),
                'v' => (*x, *y + 1),
                _ => panic!(),
            };

            *x = 1 + (new_x - 1).rem_euclid(w - 2);
            *y = 1 + (new_y - 1).rem_euclid(h - 2);
        }
    }

    let mut time = 0;
    time = bfs(&fields, &(1, 0, time), &(w - 2, h - 1), fields.len() as i32).unwrap();
    let part1 = time;
    time = bfs(&fields, &(w - 2, h - 1, time), &(1, 0), fields.len() as i32).unwrap();
    time = bfs(&fields, &(1, 0, time), &(w - 2, h - 1), fields.len() as i32).unwrap();

    println!("Part 1 took {part1}\nPart 2 took {time}")
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines);
}
