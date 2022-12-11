use std::io::prelude::*;
use std::env;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day10;
pub mod day11;

fn main() {
    //day1::day1();
    let args: String = env::args().collect::<Vec<String>>()[1..].join(" ");
    let lines = std::io::stdin().lock().lines().flatten();
    match &*args {
	"1 2" => day1::part2(lines),
	"4 1" => day4::part1(lines),
	"4 2" => day4::part2(std::io::stdin().lock().lines().flatten()),
	"10" => day10::func(lines),
	"11" => day11::func(lines),
	_ => println!("unknown day {args}\n"),
    };
}
