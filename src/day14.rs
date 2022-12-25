use crate::canvas::Canvas;
use itertools::Itertools;
use std::cmp::max;

impl Canvas {
    fn step_sand(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;

        if self.get_pixel(x, y) != 0 {
            return false;
        }

        while if y + 1 >= self.height {
            return false;
        } else if self.get_pixel(x, y + 1) == 0 {
            y += 1;
            true
        } else if self.get_pixel(x - 1, y + 1) == 0 {
            y += 1;
            x -= 1;
            true
        } else if self.get_pixel(x + 1, y + 1) == 0 {
            y += 1;
            x += 1;
            true
        } else {
            false
        } {}

        self.set_pixel(x, y, 5);
        return true;
    }
}

fn func(lines: impl Iterator<Item = String>, part2: bool) {
    let mut canvas = Canvas::new(1000, 200);
    let mut max_y = 0;
    for line in lines {
        let coords = line.split(" -> ").map(|x| {
            x.split(",")
                .map(|y| y.parse())
                .flatten()
                .next_tuple::<(i32, i32)>()
        });

        for (start, end) in coords.flatten().tuple_windows() {
            if start.0 == end.0 {
                canvas.vline(start.0, start.1, end.1, 1);
            } else if start.1 == end.1 {
                canvas.hline(start.0, end.0, start.1, 1);
            }
            max_y = max(max_y, start.1);
        }
    }

    if part2 {
        for x in 0..canvas.width {
            canvas.set_pixel(x, max_y + 2, 1);
        }
    }

    let mut count = 0;
    while canvas.step_sand() {
        count += 1;
    }
    // canvas.display(0, 0, 1000, 200);

    println!("{count} {max_y}")
}
pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, false);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, true);
}
