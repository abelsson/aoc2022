use itertools::Itertools;
use std::mem;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: (String, String, String),
    div_by: i64,
    true_dest: i64,
    false_dest: i64,
    inspections: i64,
}

fn parse_monkey(t: &Vec<String>, start: usize) -> (usize, Monkey) {
    let idx_after = |token: &str, i: usize| {
	i + t[i..].iter().take_while(|x| *x != token).count() + 1
    };

    let i0 = idx_after("items", start);
    let i1 = idx_after("=", i0);
    let i2 = idx_after("by", i1);
    let i3 = idx_after("monkey", i2);
    let i4 = idx_after("monkey", i3);

    let res = Monkey {
	items: t[i0..i1].iter()
	    .filter_map(|x| x.parse::<i64>().ok())
	    .collect(),
	op: t[i1..].iter().map(String::to_string).next_tuple().unwrap(),
	div_by: t[i2].parse::<i64>().unwrap(),
	true_dest: t[i3].parse::<i64>().unwrap(),
	false_dest: t[i4].parse::<i64>().unwrap(),
	inspections: 0,
    };

    return (i4 + 1, res);
}

pub fn func(lines: impl Iterator<Item = String>, iterations: i32, calc: fn(i64, i64) -> i64) {
    let mut monkeys: Vec<Monkey> = Default::default();
    let mut tokens: Vec<String> = Default::default();

    for line in lines {
	line.split(&[' ', ':', ','])
	    .filter(|x| x.len() > 0)
	    .for_each(|elem| tokens.push(elem.to_owned()));
    }

    let mut start = 0;
    while start < tokens.len() {
	let monkey;
	(start, monkey) = parse_monkey(&tokens, start);
	monkeys.push(monkey);
    }

    let gcd: i64 = monkeys.iter().map(|m| m.div_by).product();
    for _ in 0..iterations {
	for idx in 0..monkeys.len() {
	    for item in mem::take(&mut monkeys[idx].items) {
		let value = |s: &String| match s.as_str() {
		    "old" => item,
		    x => x.parse::<i64>().unwrap(),
		};
		let a = value(&monkeys[idx].op.0);
		let b = value(&monkeys[idx].op.2);
		let res = calc(
		    match monkeys[idx].op.1.as_str() {
			"*" => a * b,
			"+" => a + b,
			_ => panic!(),
		    },
		    gcd,
		);
		monkeys[idx].inspections += 1;

		if res % monkeys[idx].div_by == 0 {
		    let dest = monkeys[idx].true_dest as usize;
		    monkeys[dest].items.push(res);
		} else {
		    let dest = monkeys[idx].false_dest as usize;
		    monkeys[dest].items.push(res);
		}
	    }
	}
    }

    let res: i64 = monkeys
	.iter()
	.map(|m| m.inspections)
	.sorted()
	.rev()
	.take(2)
	.product();

    println!("{}", res);
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 20, |a, _| a / 3);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, 10000, |a, b| a % b);
}
