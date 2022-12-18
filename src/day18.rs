use itertools::{iproduct, Itertools};
use std::collections::VecDeque;

struct Canvas {
    pixels: Vec<i32>,
    height: i32,
    width: i32,
    depth: i32,
}

impl Canvas {
    fn new(width: i32, height: i32, depth: i32) -> Canvas {
        Canvas {
            pixels: vec![0; (height * width * depth) as usize],
            height,
            width,
            depth,
        }
    }

    fn get_pixel(&self, x: i32, y: i32, z: i32) -> i32 {
        return self.pixels[z as usize * (self.width * self.height) as usize
            + y as usize * self.width as usize
            + x as usize];
    }
    fn set_pixel(&mut self, x: i32, y: i32, z: i32, value: i32) {
        self.pixels[z as usize * (self.width * self.height) as usize
            + y as usize * self.width as usize
            + x as usize] = value;
    }

    fn neighbours(&self, x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
        let offsets = vec![
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];

        return offsets
            .iter()
            .map(|c| (x + c.0, y + c.1, z + c.2))
            .filter(|c| {
                c.0 >= 0
                    && c.1 >= 0
                    && c.2 >= 0
                    && c.0 < self.width
                    && c.1 < self.height
                    && c.2 < self.depth
            })
            .collect();
    }
}

fn flood_fill(canvas: &mut Canvas) {
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 0));

    while let Some(current) = queue.pop_front() {
        let (x, y, z) = current;
        if canvas.get_pixel(x, y, z) == 0 {
            canvas.set_pixel(x, y, z, 2);

            canvas
                .neighbours(x, y, z)
                .iter()
                .for_each(|pos| queue.push_back(*pos));
        }
    }
}
fn func(lines: impl Iterator<Item = String>, part2: bool) {
    let mut canvas = Canvas::new(100, 100, 100);

    for line in lines {
        let (x, y, z) = line
            .split(",")
            .map(|y| y.parse())
            .flatten()
            .next_tuple::<(i32, i32, i32)>()
            .unwrap();

        canvas.set_pixel(x, y, z, 1);
    }

    let mut sum = 0;
    for (z, y, x) in iproduct!(0..canvas.depth, 0..canvas.height, 0..canvas.width) {
        if canvas.get_pixel(x, y, z) != 0 {
            let count: i32 = canvas
                .neighbours(x, y, z)
                .iter()
                .map(|(x, y, z)| canvas.get_pixel(*x, *y, *z))
                .sum();

            sum += 6 - count;
            //println!("{x} {y} {z}: {count}")
        }
    }

    if part2 {
        flood_fill(&mut canvas);
        for (z, y, x) in iproduct!(0..canvas.depth, 0..canvas.height, 0..canvas.width) {
            if canvas.get_pixel(x, y, z) == 0 {
                let count: i32 = canvas
                    .neighbours(x, y, z)
                    .iter()
                    .map(|(x, y, z)| (canvas.get_pixel(*x, *y, *z) == 1) as i32)
                    .sum();

                sum -= count;
                //println!("{x} {y} {z}: {count}");
            }
        }
    }

    println!("{sum}");
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, false);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, true);
}
