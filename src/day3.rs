use std::io::prelude::*;
use std::collections::HashSet;
use itertools::Itertools;

fn score(data: &str) -> u32 {

    let a = 'a' as u32 - 1;
    let A = 'A' as u32 - 27;

    let mid = data.len() / 2;
    let (first, second) = data.split_at(mid);

    let set_a : HashSet<char> = HashSet::from_iter(first.chars());
    let set_b = HashSet::from_iter(second.chars());
    let sum = set_a.intersection(&set_b)
	.map(|ch| *ch as u32 - if ch.is_lowercase() { a } else { A })
	.sum();

    return sum;
}

fn score2(lines: impl Iterator<Item = impl Into<String>>) -> u32 {

    let a = 'a' as u32 - 1;
    let A = 'A' as u32 - 27;

    let v : Vec<HashSet<char>> = lines.map(|x| HashSet::from_iter(x.into().chars())).collect();

    let sum = HashSet::from_iter(v[0].intersection(&v[1]).copied()).intersection(&v[2])
	.map(|ch| *ch as u32 - if ch.is_lowercase() { a } else { A })
	.sum();

    return sum;
}

pub fn func() {
    let res: u32 = std::io::stdin().lock().lines().flatten()
	.map(|elt| score(&elt)).sum();

    println!("{res}");
}

pub fn func2(lines: impl Iterator<Item = impl Into<String>>) {
    let res: u32 = lines.chunks(3).into_iter()
	.map(|elt| score2(elt)).sum();

    println!("{res}");
}
