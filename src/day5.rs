use itertools::Itertools;
use std::str;

pub fn func(lines_: impl Iterator<Item = String>) {
    let tmp = lines_.group_by(|x| x.len() > 0);
    let sections : Vec<Vec<String>> = tmp.into_iter().map(|(_, x)| x.collect::<Vec<_>>()).collect();

    let mut stacks: Vec<Vec<String>> = vec![Default::default(); 10];

    for line in &sections[0] {
	line.as_bytes()
	    .chunks(4)
	    .into_iter()
	    .map(str::from_utf8)
	    .enumerate()
	    .filter(|(_, x)| !x.unwrap().trim().is_empty())
	    .for_each(|(idx, x)| stacks[idx + 1].insert(0, x.unwrap().to_string()));
    }

    for line in &sections[2] {
	let (amount, from, to): (usize, usize, usize) = line
	    .split_whitespace()
	    .map(|x| x.parse())
	    .flatten()
	    .next_tuple()
	    .unwrap();

	let start = stacks[from].len() - amount;
	let elems: Vec<String> = stacks[from][start..].to_vec();
	stacks[to].extend(elems);
	stacks[from].truncate(start);
    }

    let res = stacks.into_iter()
	.filter(|x| x.len() > 0)
	.map(|x| x.last().unwrap().chars().nth(1).unwrap()).join("");
    println!("{:?}", res);
}
