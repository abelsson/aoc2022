use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

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
                print!("{pixel} ");
            }
            println!();
        }
    }

    fn neighbours4(&self, x: i32, y: i32, diagonals: bool) -> Vec<(i32, i32)> {
        let offsets = if diagonals {
            vec![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
        } else {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        };

        return offsets
            .iter()
            .map(|c| (x + c.0, y + c.1))
            .filter(|c| c.0 >= 0 && c.1 >= 0 && c.0 < self.width && c.1 < self.height)
            .collect();
    }
}

fn turn_func(dir: char, turn: char) -> char {
    match (dir, turn) {
        ('R', 'R') => 'D',
        ('D', 'R') => 'L',
        ('L', 'R') => 'U',
        ('U', 'R') => 'R',
        ('R', 'L') => 'U',
        ('U', 'L') => 'L',
        ('L', 'L') => 'D',
        ('D', 'L') => 'R',
        (_, _) => dir,
    }
}
fn func(lines_it: impl Iterator<Item = String>, part2: bool) {
    let mut canvas = Canvas::new(500, 500);
    let lines = lines_it.collect_vec();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            match *ch as char {
                '.' => canvas.set_pixel(x as i32 + 1, y as i32 + 1, 1),
                '#' => canvas.set_pixel(x as i32 + 1, y as i32 + 1, 2),
                _ => (),
            }
        }
    }

    let mut direction = 'R';
    let mut y = 1;
    let mut x = 1;
    while canvas.get_pixel(x, y) == 0 {
        x += 1;
    }

    let directions = &lines[lines.len() - 1];

    println!("start: {x} {y}");
    canvas.set_pixel(x, y, 5);
    println!("{directions}");
    let ds = directions.split_inclusive(&['R', 'L']).collect_vec();

    let mut count = 5;
    for d in ds {
        let turn = d.chars().last().unwrap();
        let amount = d
            .replace("R", "")
            .replace("L", "")
            .replace("X", "")
            .parse()
            .unwrap();
        let delta = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!(),
        };
        println!("Cmd {amount} {d}: ({direction}) pos: {x}, {y}");
        for _ in 0..amount {
            let (dx, dy) = delta;
            let px = canvas.get_pixel(x + dx, y + dy);
            if px == 2 {
                println!("Hit rock, stopping at {x} {y}");
                break;
            }

            if px == 0 {
                // wrap
                let prev_x = x;
                let prev_y = y;
                match direction {
                    'R' => {
                        x = 0;
                        while canvas.get_pixel(x, y) == 0 {
                            x += 1;
                        }
                    }
                    'L' => {
                        x = canvas.width - 1;
                        while canvas.get_pixel(x, y) == 0 {
                            x -= 1;
                        }
                    }
                    'U' => {
                        y = canvas.height - 1;
                        while canvas.get_pixel(x, y) == 0 {
                            y -= 1;
                        }
                    }
                    'D' => {
                        y = 0;
                        while canvas.get_pixel(x, y) == 0 {
                            y += 1;
                        }
                    }
                    _ => panic!(),
                };
                if canvas.get_pixel(x, y) == 2 {
                    x = prev_x;
                    y = prev_y;
                    break;
                }
                println!("Wrap to {x}, {y} {}", canvas.get_pixel(x, y));
                assert!(canvas.get_pixel(x, y) == 1 || canvas.get_pixel(x, y) >= 5);
                //assert!(canvas.get_pixel(x, y) == 1);
                canvas.set_pixel(x, y, count);
                continue;
            }

            x = x + dx;
            y = y + dy;
            assert!(canvas.get_pixel(x, y) == 1 || canvas.get_pixel(x, y) >= 5);
            canvas.set_pixel(x, y, count);
            println!("Move {direction}: {x}, {y} (dx={dx}, dy={dy})");
        }
        direction = turn_func(direction, turn);
        println!("Turn {turn} to {direction}");
        if count < 9 {
            count += 1;
        }
    }
    canvas.display(0, 0, 200, 200);

    let res = 4 * x
        + 1000 * y
        + match direction {
            'R' => 0,
            'D' => 1,
            'L' => 2,
            'U' => 3,
            _ => panic!(),
        };
    println!("{x} {y} {direction} {res}");
}

pub fn part1(lines: impl Iterator<Item = String>) {
    // 40240
    func(lines, false);
}

pub fn part2(lines_it: impl Iterator<Item = String>) {
    let mut canvas = Canvas::new(500, 500);
    let mut faces = vec![Canvas::new(100, 100); 6];
    let lines = lines_it.collect_vec();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.as_bytes().iter().enumerate() {
            match *ch as char {
                '.' => canvas.set_pixel(x as i32 + 1, y as i32 + 1, 1),
                '#' => canvas.set_pixel(x as i32 + 1, y as i32 + 1, 2),
                _ => (),
            }
        }
    }

    let face_size = 50;

    let face_locs = [(1, 0), (2, 0), (1, 1), (0, 2), (1, 2), (0, 3)];
    for (idx, face_id) in face_locs.iter().enumerate() {
        let sx = face_id.0 * face_size;
        let sy = face_id.1 * face_size;

        for (dy, y) in (sy..(sy + face_size)).enumerate() {
            for (dx, x) in (sx..(sx + face_size)).enumerate() {
                faces[idx].set_pixel(dx as i32 + 1, dy as i32 + 1, canvas.get_pixel(x + 1, y + 1));
            }
        }

        println!("Face {}:", idx + 1);

        faces[idx].display(0, 0, face_size + 2, face_size + 2);
    }

    // top, right, down, left
    let neighbours = [
        [2, 6, 4, 3],
        [1, 3, 5, 6],
        [1, 4, 5, 1],
        [1, 6, 5, 3],
        [5, 4, 3, 6],
        [1, 2, 4, 5],
    ];

    let mut direction = 'R';
    let mut y = 1;
    let mut x = 1;
    let mut face = 0;
    while faces[face].get_pixel(x, y) == 0 {
        x += 1;
    }

    let directions = &lines[lines.len() - 1];

    println!("start: {x} {y}");
    canvas.set_pixel(x, y, 5);
    println!("{directions}");
    let ds = directions.split_inclusive(&['R', 'L']).collect_vec();

    let mut count = 5;
    for d in ds {
        let turn = d.chars().last().unwrap();
        let amount = d
            .replace("R", "")
            .replace("L", "")
            .replace("X", "")
            .parse()
            .unwrap();

        println!("Cmd {amount} {d}: ({direction}) pos: {x}, {y}");
        for _ in 0..amount {
            let delta = match direction {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, -1),
                'D' => (0, 1),
                _ => panic!(),
            };
            let (dx, dy) = delta;
            let px = faces[face].get_pixel(x + dx, y + dy);
            if px == 2 {
                println!("Hit rock, stopping at {x} {y}");
                break;
            }

            if px == 0 {
                // wrap
                let prev_x = x;
                let prev_y = y;

                let w = face_size;
                let m = |x| w + 1 - x;
                let (new_face, new_dir, new_x, new_y) = match (face + 1, direction) {
                    (1, 'U') => (6, 'R', 1, x),
                    (1, 'R') => (2, 'R', 2, y),
                    (1, 'D') => (3, 'D', x, 1),
                    (1, 'L') => (4, 'R', 1, m(y)),
                    (2, 'U') => (6, 'U', x, w),
                    (2, 'R') => (5, 'L', w, m(y)),
                    (2, 'D') => (3, 'L', w, x),
                    (2, 'L') => (1, 'L', w, y),
                    (3, 'U') => (1, 'U', x, w),
                    (3, 'R') => (2, 'U', y, w),
                    (3, 'D') => (5, 'D', x, 1),
                    (3, 'L') => (4, 'D', y, 1),
                    (4, 'U') => (3, 'R', 1, x),
                    (4, 'R') => (5, 'R', 1, y),
                    (4, 'D') => (6, 'D', x, 1),
                    (4, 'L') => (1, 'R', 1, m(y)),
                    (5, 'U') => (3, 'U', x, w),
                    (5, 'R') => (2, 'L', w, m(y)),
                    (5, 'D') => (6, 'L', w, x),
                    (5, 'L') => (4, 'L', w, y),
                    (6, 'U') => (4, 'U', x, w),
                    (6, 'R') => (5, 'U', y, w),
                    (6, 'D') => (2, 'D', x, 1),
                    (6, 'L') => (1, 'D', y, 1),
                    (_, _) => panic!(),
                };
                println!(
                    "WRAP from {} ({x} {y}) to face {} ({new_x}, {new_y}) {} {new_dir}",
                    face + 1,
                    new_face,
                    faces[new_face - 1].get_pixel(new_x, new_y)
                );

                if faces[new_face - 1].get_pixel(new_x, new_y) == 2 {
                    println!("Hit rock, stopping at {x} {y}");
                    break;
                }
                face = new_face - 1;
                direction = new_dir;
                x = new_x;
                y = new_y;
                assert!(faces[face].get_pixel(x, y) == 1 || faces[face].get_pixel(x, y) >= 5);
                faces[face].set_pixel(x, y, count);
                continue;
            }

            x = x + dx;
            y = y + dy;
            assert!(faces[face].get_pixel(x, y) == 1 || faces[face].get_pixel(x, y) >= 5);
            faces[face].set_pixel(x, y, count);
            println!(
                "Move {direction}: face: {} ({x}, {y}) (dx={dx}, dy={dy})",
                { face + 1 }
            );
        }
        direction = turn_func(direction, turn);
        println!("Turn {turn} to {direction}");
    }

    for idx in 0..6 {
        println!("Face {}:", idx + 1);
        faces[idx].display(0, 0, face_size + 2, face_size + 2);
    }

    let face_id = face_locs[face];
    let sx = face_id.0 * face_size;
    let sy = face_id.1 * face_size;

    let res = 4 * (sx + x)
        + 1000 * (sy + y)
        + match direction {
            'R' => 0,
            'D' => 1,
            'L' => 2,
            'U' => 3,
            _ => panic!(),
        };
    println!(
        "face: {} | {sx} + {x}, {sy} + {y} {direction} {res}",
        face + 1
    );

    /*

       1 U -> 6 R, x = 1, y = x
       1 R -> 2 R, x = 2, y = y
       1 D -> 3 D, x = x, y = 1
       1 L -> 4 R, x = 1, y =!y
       2 U -> 6 U, x = x, y = w
       2 R -> 5 L, x = w, y =!y
       2 D -> 3 L, x = w, y = x
       2 L -> 1 L, x = w, y = y
       3 U -> 1 U, x = x, y = w
       3 R -> 2 U, x = y, y = w
       3 D -> 5 D, x = x, y = 1
       3 L -> 4 D, x = y, y = 1
       4 U -> 3 R, x = 1, y = y
       4 R -> 5 R, x = 1, y = y
       4 D -> 6 D, x = x, y = 1
       4 L -> 1 R, x = 1, y =!y
       5 U -> 3 U, x = x, y = w
       5 R -> 2 L, x = w, y =!y
       5 D -> 6 L, x = w, y = x
       5 L -> 4 L, x = w, y = y
       6 U -> 4 U, x = x, y = w
       6 R -> 5 U, x = y, y = w
       6 D -> 2 D, x = x, y = 1
       6 L -> 1 D, x = y, y = 1
    */
}
