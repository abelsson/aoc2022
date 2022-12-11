#[derive(Debug, Clone)]
struct Monkey{
    idx : i64,
    items : Vec<i64>,
    opa : String,
    op: String,
    opb : String,

    div_by : i64,
    true_dest : i64,
    false_dest : i64,
    inspections : i64,
}

fn parse_monkey(t : &Vec<String>, start : usize) -> (usize, Option<Monkey>) {

    let mut i = start;
    while t[i] != "Monkey" {
	i+=1;
	if i >= t.len() {
	    return (t.len(), None);
	}
    }
    i += 1;
    let monkey : i64 = t[i].parse().unwrap();
    while t[i] != "items" { i+=1; }
    let mut items: Vec<i64> = vec![];

    while t[i] != "=" {
	if let Ok(num) = t[i].parse::<i64>() {
	    items.push(num);
	}
	i += 1;
    }
    i += 1; //
    let opa = &t[i]; i += 1;
    let op = &t[i]; i += 1;
    let opb = &t[i]; i += 1;
    dbg!(monkey);
    dbg!(opa, op, opb);

    while t[i] != "by" { i+=1; } i+=1;

    let div_by = t[i].parse::<i64>().unwrap();
    while t[i] != "monkey" { i+=1; } i+=1;
    let true_dest = t[i].parse::<i64>().unwrap();
    while t[i] != "monkey" { i+=1; } i+=1;
    let false_dest = t[i].parse::<i64>().unwrap();

    dbg!(div_by, true_dest, false_dest);

    let res = Monkey{ idx: monkey, items: items,
		      opa: opa.to_string(), op: op.to_string(), opb: opb.to_string(),
		      div_by: div_by,
		      true_dest: true_dest, false_dest: false_dest,
		      inspections: 0};
    dbg!(&res);
    return (i, Some(res));
}
pub fn func(lines: impl Iterator<Item = String>) {

    let mut monkeys: Vec<Monkey> = Default::default();
    let mut tokens: Vec<String> = Default::default();
    for line in lines {
	let elems = line.split(&[' ',':',',']).collect::<Vec<&str>>();
	for elem in elems {
	    tokens.push(elem.to_owned());
	}
    }

/*
    for token in &tokens {
	println!("{token}");
    }
     */
    let mut start = 0;
    while start < tokens.len() {
	let monkey;
	(start, monkey) = parse_monkey(&tokens, start);
	if let Some(m) = monkey {
	    monkeys.push(m);
	}
    }
    dbg!(&monkeys);

    let gcd: i64 = monkeys.iter().map(|m| m.div_by).product();
    dbg!(gcd);
    for round in 0..10000 {

	for idx in 0..monkeys.len() {

	    let items = monkeys[idx].items.clone();
	    monkeys[idx].items.clear();
	    for item in items {
		let a = match monkeys[idx].opa.as_str() {
		    "old" => item,
		    x => x.parse::<i64>().unwrap(),
		};
		let b = match monkeys[idx].opb.as_str() {
		    "old" => item,
		    x => x.parse::<i64>().unwrap(),
		};
		let res = match monkeys[idx].op.as_str() {
		    "*" => (a * b) % gcd,
		    "+" => (a + b) % gcd,
		    "-" => a - b,
		    _ => panic!()
		};
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

    let mut insps: Vec<i64> = Vec::new();

    for m in &monkeys {
	insps.push(m.inspections);
    }
    dbg!(&monkeys);
    insps.sort();
    dbg!(&insps);


}
