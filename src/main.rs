#![allow(dead_code)]
use std::env;
use std::io::prelude::*;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn main() {
    //day1::day1();
    let args: String = env::args().collect::<Vec<String>>()[1..].join(" ");
    let lines = std::io::stdin().lock().lines().flatten();
    match &*args {
        "1 2" => day1::part2(lines),
        "3 2" => day3::func2(lines),
        "4 1" => day4::part1(lines),
        "4 2" => day4::part2(lines),
        "5" => day5::func(lines),
        "6 1" => day6::part1(lines),
        "6 2" => day6::part2(lines),
        "7" => day7::func(lines),
        "8" => day8::func(lines),
        "9 1" => day9::part1(lines),
        "9 2" => day9::part2(lines),
        "10" => day10::func(lines),
        "11 1" => day11::part1(lines),
        "11 2" => day11::part2(lines),
        "12 1" => day12::part1(lines),
        "12 2" => day12::part2(lines),
        "13 1" => day13::part1(lines),
        "13 2" => day13::part2(lines),
        "14 1" => day14::part1(lines),
        "14 2" => day14::part2(lines),
        "15 1" => day15::part1(lines),
        "15 2" => day15::part2(lines),
        "16 1" => day16::part1(lines),
        "16 2" => day16::part2(lines),
        "17 1" => day17::part1(lines),
        "17 2" => day17::part2(lines),
        "18 1" => day18::part1(lines),
        "18 2" => day18::part2(lines),
        "19 1" => day19::part1(lines),
        "19 2" => day19::part2(lines),
        "20 1" => day20::part1(lines),
        "20 2" => day20::part2(lines),
        "21 1" => day21::part1(lines),
        "21 2" => day21::part2(lines),
        "22 1" => day22::part1(lines),
        "22 2" => day22::part2(lines),
        "23 1" => day23::part1(lines),
        "23 2" => day23::part2(lines),
        "24 1" => day24::part1(lines),
        "24 2" => day24::part2(lines),
        _ => println!("unknown day {args}\n"),
    };
}
