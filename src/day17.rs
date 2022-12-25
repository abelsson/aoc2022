use crate::canvas::Canvas;

fn first_set_line(canvas: &Canvas, _start_y: i32) -> i32 {
    for y in 0..canvas.height {
        let mut any_set = false;
        for x in 0..canvas.width {
            any_set |= canvas.get_pixel(x, y) != 0;
        }
        if any_set {
            return y;
        }
    }
    canvas.height
}

fn func(mut lines: impl Iterator<Item = String>) {
    let mut canvas = Canvas::new(7, 8000);

    let shape1 = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let shape2 = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    let shape3 = vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let shape4 = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let shape5 = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

    let shapes = vec![shape1, shape2, shape3, shape4, shape5];
    let shape_width = vec![4, 3, 3, 1, 2];
    let shape_height = vec![1, 3, 3, 4, 2];

    let line = lines.next().unwrap();
    let wind = line.as_bytes();
    let mut wind_idx = 0;

    let mut prev_h = 0;

    let mut deltas = Vec::new();

    for shape_idx in 0..4000 {
        let shape = &shapes[shape_idx % 5];
        let w = shape_width[shape_idx % 5];
        let h = shape_height[shape_idx % 5];

        //println!("New shape {shape_idx}");
        let mut x = 2;
        let start_height = first_set_line(&canvas, 0) - h - 3;
        let mut y = start_height;

        //println!("Start height {start_height}");

        while y < canvas.height {
            match wind[wind_idx] as char {
                '>' => {
                    if x + w < 7 && !collides(x + 1, y, h, &canvas, shape) {
                        x += 1;
                    }
                }
                '<' => {
                    if x > 0 && !collides(x - 1, y, h, &canvas, shape) {
                        x -= 1;
                    }
                }
                _ => panic!(),
            }
            //println!("{}", wind[wind_idx] as char);
            wind_idx = (wind_idx + 1) % wind.len();

            y += 1;
            let collides = collides(x, y, h, &canvas, shape);
            if collides {
                shape
                    .iter()
                    .for_each(|c| canvas.set_pixel(x + c.0, y + c.1 - 1, 5));
                //canvas.display(0, 0, 7, 40);

                break;
            }
        }

        let h = canvas.height - first_set_line(&canvas, 0);
        print! {"{}", h - prev_h};
        deltas.push(h - prev_h);
        prev_h = h;
    }

    println!();
    /*
    let window = 5;
    for idx in 1..(deltas.len()-window) {
        if deltas[idx..(idx+window)].eq(&deltas[0..window]) {
            println!("Cycle at {idx}?");
            dbg!(deltas[idx..(idx+window)].iter());
            dbg!(deltas[0..window].iter());
        }
    }
    */
    let first_set = canvas.height - first_set_line(&canvas, 0);

    println!("{first_set} {}", wind.len())
}

fn collides(x: i32, y: i32, h: i32, canvas: &Canvas, shape: &Vec<(i32, i32)>) -> bool {
    y + h > canvas.height
        || shape
            .iter()
            .any(|c| canvas.get_pixel(x + c.0, y + c.1) != 0)
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    //(1000000000000-1180)/1740 = 574712643
    // sum first 1740 elements = 1878
    // sum cycle of 1740 elements = 2759
    // ans = 574712643*2759+1878
    func(lines);
}
