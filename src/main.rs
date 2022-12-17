use std::env;
use std::io::prelude::*;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

fn main() {
    //day1::day1();
    let args: String = env::args().collect::<Vec<String>>()[1..].join(" ");
    let lines = std::io::stdin().lock().lines().flatten();
    match &*args {
	"1 2" => day1::part2(lines),
	"4 1" => day4::part1(lines),
	"4 2" => day4::part2(lines),
	"10" => day10::func(lines),
	"11 1" => day11::part1(lines),
	"11 2" => day11::part2(lines),
	"12 1" => day12::part1(lines),
	"12 2" => day12::part2(lines),
	"14 1" => day14::part1(lines),
	"14 2" => day14::part2(lines),
	"15 1" => day15::part1(lines),
	"15 2" => day15::part2(lines),
	"16 1" => day16::part1(lines),
	"16 2" => day16::part2(lines),

	_ => println!("unknown day {args}\n"),
    };
}
