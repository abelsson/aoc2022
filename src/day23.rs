use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, VecDeque},
};

use crate::canvas::Canvas;

fn func(lines: impl Iterator<Item = String>, max_rounds: i32) {
    let mut canvas = Canvas::new(1000, 1000);

    let offset = 400;
    for (y, line) in lines.enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            match *ch as char {
                '.' => canvas.set_pixel(x as i32 + offset, y as i32 + offset, 0),
                '#' => canvas.set_pixel(x as i32 + offset, y as i32 + offset, 1),
                _ => (),
            }
        }
    }

    let mut eval_order = VecDeque::from([2, 3, 4, 5]);
    let mut rounds = 0;
    let mut any_moved = true;
    while any_moved && rounds < max_rounds {
        any_moved = false;
        let mut proposed_moves: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                let neighbour_sum = |x, y, typ| {
                    canvas
                        .neighbours4(x, y, typ)
                        .iter()
                        .map(|(x, y)| canvas.get_pixel(*x, *y))
                        .sum::<i32>()
                };
                if canvas.get_pixel(x, y) == 1 {
                    if neighbour_sum(x, y, 1) == 0 {
                        continue;
                    }
                    for idx in 0..4 {
                        let neighbours = canvas.neighbours4(x, y, eval_order[idx % 4]);
                        if neighbour_sum(x, y, eval_order[idx]) > 0 {
                            continue;
                        }
                        proposed_moves.insert((x, y), neighbours[1]);
                        break;
                    }
                }
            }
        }

        let mut duplicates: HashMap<(i32, i32), i32> = HashMap::new();

        for k in proposed_moves.values() {
            duplicates.entry(*k).and_modify(|e| *e += 1).or_insert(1);
        }

        for (start, dest) in proposed_moves
            .iter()
            .filter(|&(_, v)| *duplicates.get(v).unwrap() == 1)
        {
            canvas.set_pixel(start.0, start.1, 0);
            canvas.set_pixel(dest.0, dest.1, 1);
            any_moved = true;
        }

        // println!("");

        // canvas.display(0, 0, 20, 20);
        eval_order.rotate_left(1);
        // println!("{:?}", eval_order);
        rounds += 1;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            if canvas.get_pixel(x, y) == 1 {
                min_x = min(min_x, x);
                max_x = max(max_x, x);
                min_y = min(min_y, y);
                max_y = max(max_y, y);
            }
        }
    }

    canvas.display(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1);
    let res = canvas.count_equal(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1, 0);
    println!("{res} {rounds}");
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 10);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, i32::MAX);
}
