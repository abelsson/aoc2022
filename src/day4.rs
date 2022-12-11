use itertools::Itertools;
use std::ops::RangeInclusive;

fn contained_in(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    return range1.start() >= range2.start() && range1.end() <= range2.end();
}

fn ranges_disjoint(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> bool {
    return range1.start() > range2.end() || range1.end() < range2.start();
}

fn score(input: &str) -> u32 {
    let (s1, e1, s2, e2) = input
        .split(&[',', '-'])
        .map(|x| x.parse())
        .flatten()
        .next_tuple()
        .unwrap();
    let range1 = s1..=e1;
    let range2 = s2..=e2;

    return (contained_in(&range1, &range2) || contained_in(&range2, &range1)) as u32;
}

fn score2(input: &str) -> u32 {
    let (s1, e1, s2, e2) = input
        .split(&[',', '-'])
        .map(|x| x.parse())
        .flatten()
        .next_tuple()
        .unwrap();

    return !ranges_disjoint(&(s1..=e1), &(s2..=e2)) as u32;
}

pub fn part1(lines: impl Iterator<Item = impl Into<String>>) {
    let res: u32 = lines.map(|elt| score(&elt.into())).sum();
    println!("{res}");
}

pub fn part2(lines: impl Iterator<Item = impl Into<String>>) {
    let res: u32 = lines.map(|elt| score2(&elt.into())).sum();
    println!("{res}");
}
