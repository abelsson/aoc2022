use itertools::Itertools;
use std::io::prelude::*;

pub fn part2(_lines: impl Iterator<Item = impl Into<String>>) {
    let res: Vec<i64> = std::io::stdin()
        .lock()
        .lines()
        .flatten()
        // Group by empty lines
        .group_by(|elt| (*elt).is_empty())
        .into_iter()
        // parse and sum each group
        .map(|(_, group)| group.flat_map(|elt| elt.parse::<i64>()).sum())
        // sort results (largest first) and take top 3
        .sorted()
        .rev()
        .take(3)
        .collect();

    println!("{} {}", res[0], res.iter().sum::<i64>());
}
