use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, VecDeque},
};

use itertools::Itertools;

struct Canvas {
    pixels: Vec<i32>,
    height: i32,
    width: i32,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Canvas {
        Canvas {
            pixels: vec![0; (height * width) as usize],
            height: height,
            width: width,
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> i32 {
        return self.pixels[y as usize * self.width as usize + x as usize];
    }
    fn set_pixel(&mut self, x: i32, y: i32, value: i32) {
        self.pixels[y as usize * self.width as usize + x as usize] = value;
    }

    fn hline(&mut self, x1: i32, x2: i32, y: i32, value: i32) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

        for x in start..=end {
            self.set_pixel(x, y, value);
        }
    }

    fn vline(&mut self, x: i32, y1: i32, y2: i32, value: i32) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        for y in start..=end {
            self.set_pixel(x, y, value);
        }
    }

    fn display(&self, sx: i32, sy: i32, width: i32, height: i32) {
        println!("P2 {width} {height} 5");
        for y in sy..(sy + height) {
            for x in sx..(sx + width) {
                let pixel = self.get_pixel(x, y);
                match pixel {
                    0 => print!("."),
                    1 => print!("#"),
                    x => print!("{x} "),
                }
            }
            println!();
        }
    }

    fn count_equal(&self, sx: i32, sy: i32, width: i32, height: i32, value: i32) -> i32 {
        let mut count = 0;
        for y in sy..(sy + height) {
            for x in sx..(sx + width) {
                if self.get_pixel(x, y) == value {
                    count += 1
                }
            }
        }
        return count;
    }
    fn neighbours4(&self, x: i32, y: i32, typ: i32) -> Vec<(i32, i32)> {
        let offsets = match typ {
            0 => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            1 => vec![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ],
            2 => vec![(-1, -1), (0, -1), (1, -1)], // north
            3 => vec![(-1, 1), (0, 1), (1, 1)],    // south
            4 => vec![(-1, 1), (-1, 0), (-1, -1)], // west
            5 => vec![(1, 1), (1, 0), (1, -1)],    // east
            _ => panic!(),
        };
        return offsets
            .iter()
            .map(|c| (x + c.0, y + c.1))
            .filter(|c| c.0 >= 0 && c.1 >= 0 && c.0 < self.width && c.1 < self.height)
            .collect();
    }
}

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
