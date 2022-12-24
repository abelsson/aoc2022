use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
    io,
};

use itertools::Itertools;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[derive(Clone)]
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
                    0 => print!(". "),
                    1 => print!("# "),
                    2 => print!("* "),
                    33 => print!("🙂"),
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
            0 => vec![(-1, 0), (1, 0), (0, -1), (0, 1)], // cross
            1 => vec![
                // cross + diagonals
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
                .find(|&(k, v)| k.0 == target.0 && k.1 == target.1)
                .unwrap()
                .1;
            let mut len = 0;
            let mut h: Vec<(i32, i32, i32)> = Vec::new();
            while &pos != start {
                h.push(pos);
                pos = *come_from.get(&pos).unwrap();
                len += 1;
            }
            println!("{:?}", h);

            for &(x, y, time) in h.iter().rev() {
                let mut c = map[time as usize].clone();
                let prev = c.get_pixel(x, y);
                c.set_pixel(x, y, 33);
                c.display(0, 0, c.width, c.height);
                pause();
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
fn func(lines: impl Iterator<Item = String>, max_rounds: i32) {
    let offset = 0;
    let mut W = 122; // 7, 8, 122
    let mut H = 27; // 7, 6, 27
    let mut canvas = Canvas::new(W, H);
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

    for time in 0..800 {
        let mut c = canvas.clone();

        // record current blizzard state
        for &(x, y, _) in &blizzards {
            c.set_pixel(x, y, 2);
        }
        c.display(0, 0, W, H);
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

            *x = if new_x < 1 {
                W - 2
            } else if new_x >= W - 1 {
                1
            } else {
                new_x
            };

            *y = if new_y < 1 {
                H - 2
            } else if new_y >= H - 1 {
                1
            } else {
                new_y
            };
        }
    }

    let mut time = 0;
    time = bfs(&fields, &(1, 0, time), &(W - 2, H - 1), fields.len() as i32).unwrap();
    time = bfs(&fields, &(W - 2, H - 1, time), &(1, 0), fields.len() as i32).unwrap();
    time = bfs(&fields, &(1, 0, time), &(W - 2, H - 1), fields.len() as i32).unwrap();

    println!("Took {time}")
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 10);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, i32::MAX);
}
