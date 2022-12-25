use itertools::Itertools;

fn func(lines: impl Iterator<Item = String>) {
    let mut sum = 0;
    for line in lines {
        let digits = line.as_bytes().iter().map(|ch| match *ch as char {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        });

        let mut pow = 1 as i64;
        let mut num = 0 as i64;
        for p in digits.rev() {
            num += p * pow;
            pow *= 5;
        }
        println!("{num}");
        sum += num;
    }

    println!("Sum = {sum}");
    // 8 % 5 = 3
    let mut b5digits = Vec::new();
    let sum2 = sum;
    while sum != 0 {
        b5digits.push(sum % 5);
        sum /= 5;
    }
    let mut rem = 0;
    let mut res = Vec::new();
    for &x in b5digits.iter() {
        println!("{x} + {rem}");
        res.push(match (x + rem) % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!(),
        });
        rem = 0;
        if x >= 3 {
            rem = 1;
        }
    }
    if rem == 1 {
        res.push('1');
    }

    let print = res.iter().rev().join("");
    print!("{:?} {sum2} = {print}", b5digits);
    print!("");
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines);
}
