use std::collections::VecDeque;

use itertools::Itertools;

fn func(lines: impl Iterator<Item = String>, count: i32, multiplier: i64) {
    let mut nums: VecDeque<(usize, i64)> = lines
        .map(|l| l.parse::<i64>().unwrap() * multiplier)
        .enumerate()
        .collect();

    let size = nums.len() as i64;
    for _ in 0..count {
        for idx in 0..nums.len() {
            let (src_idx, &value) = nums.iter().find_position(|(pos, _)| *pos == idx).unwrap();
            let dst_idx = value.1.rem_euclid(size - 1) as usize;
            nums.rotate_left(src_idx);
            nums.pop_front();
            nums.rotate_left(dst_idx);
            nums.push_front(value);
        }
    }

    let zero_pos = nums.iter().position(|(_, x)| *x == 0).unwrap() as i64;

    let res: i64 = (1..=3)
        .map(|n| nums[((zero_pos + n * 1000) % size) as usize].1)
        .sum();
    println!("{}", res);
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 1, 1);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, 10, 811589153);
}
