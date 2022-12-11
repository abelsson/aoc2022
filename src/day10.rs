pub fn func(lines: impl Iterator<Item = String>) {

    let mut x = 1;
    let mut cycle = 0;
    let mut delay = vec![Option::<i32>::None; 3];

    let mut sum = 0;
    for line in lines {
        let mut cmd = line.split_whitespace();

        let time = match cmd.next().unwrap() {
            "addx" => {
                let arg: i32 = cmd.next().unwrap().parse().unwrap();
                delay[2] = Some(arg);
                2
            } ,
            _ => { 1 }

        };

        for _ in 0..time {
            cycle += 1;
            delay[0] = delay[1];
            delay[1] = delay[2];
            delay[2] = None;

            if cycle == 20 || (cycle + 20) % 40 == 0 {
                sum += cycle * x;
            }

            let row = (cycle - 1) % 40;
            if x >= row - 1 && x <= row + 1 {
                print!("#");
            } else {
                print!(".");
            }
            if row == 39 {
                println!();
            }
            if let Some(value) = delay[0] {
                x += value;
            }
        }
    }
    println!("Final sum: {sum}");
}
