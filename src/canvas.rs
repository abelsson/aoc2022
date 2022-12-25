#[derive(Clone)]
pub struct Canvas {
    pixels: Vec<i32>,
    pub height: i32,
    pub width: i32,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        Canvas {
            pixels: vec![0; (height * width) as usize],
            height: height,
            width: width,
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> i32 {
        return self.pixels[y as usize * self.width as usize + x as usize];
    }
    pub fn set_pixel(&mut self, x: i32, y: i32, value: i32) {
        self.pixels[y as usize * self.width as usize + x as usize] = value;
    }

    pub fn hline(&mut self, x1: i32, x2: i32, y: i32, value: i32) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

        for x in start..=end {
            self.set_pixel(x, y, value);
        }
    }

    pub fn vline(&mut self, x: i32, y1: i32, y2: i32, value: i32) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        for y in start..=end {
            self.set_pixel(x, y, value);
        }
    }

    pub fn display(&self, sx: i32, sy: i32, width: i32, height: i32) {
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

    pub fn count_equal(&self, sx: i32, sy: i32, width: i32, height: i32, value: i32) -> i32 {
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
    pub fn neighbours4(&self, x: i32, y: i32, typ: i32) -> Vec<(i32, i32)> {
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
