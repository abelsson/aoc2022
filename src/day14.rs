use itertools::Itertools;
use std::cmp::max;

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
	let mut c: Vec<(i32, i32)> = Vec::new();

	let offsets = if (diagonals) {
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
	    x.split(",").map(|y| y.parse()).flatten().next_tuple::<(i32, i32)>()
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
