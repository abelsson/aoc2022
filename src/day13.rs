use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug)]
struct Elem {
    elem: i32,
    list: Option<Vec<Elem>>,
}

impl Elem {
    fn new(elem: i32) -> Elem {
        Elem { elem, list: None }
    }

    fn new_l(list: Vec<Elem>) -> Elem {
        Elem {
            elem: 0,
            list: Some(list),
        }
    }
}

fn compare(left: &Vec<Elem>, right: &Vec<Elem>) -> (bool, bool) {
    //dbg!(left, right);

    let mut riter = right.iter();

    let mut done;
    let mut ok;
    for l in left {
        let r = match riter.next() {
            Some(value) => value,
            None => {
                println!("Right ran out of values, not in right order");
                return (true, false);
            }
        };
        if l.list.is_some() && r.list.is_some() {
            (done, ok) = compare(&l.list.as_ref().unwrap(), &r.list.as_ref().unwrap());
            if done {
                return (done, ok);
            }
        } else if l.list.is_some() {
            println!("Mixed types, convert right and retry");
            let tmp: Vec<Elem> = vec![Elem::new(r.elem)];
            (done, ok) = compare(&l.list.as_ref().unwrap(), &tmp);
            if done {
                return (done, ok);
            }
        } else if r.list.is_some() {
            println!("Mixed types, convert left and retry");
            let tmp: Vec<Elem> = vec![Elem::new(l.elem)];
            (done, ok) = compare(&tmp, &r.list.as_ref().unwrap());
            if done {
                return (done, ok);
            }
        } else {
            println!("Compare {} vs {}", l.elem, r.elem);
            if l.elem < r.elem {
                println!("Left side is smaller, inputs are in right order");
                return (true, true);
            } else if l.elem > r.elem {
                println!("Right side is smaller, inputs are not in the right order");
                return (true, false);
            }
        }
    }

    if left.len() < right.len() {
        return (true, true);
    }
    (false, true)
}

fn parse_vec(line: &str) -> (Vec<Elem>, usize) {
    let len = line.len();
    let mut res: Vec<Elem> = Vec::new();

    let mut idx = 1usize;
    for _ in 1..len {
        match line.as_bytes()[idx] as char {
            '[' => {
                let (tmp, new_idx) = parse_vec(&line[idx..]);
                res.push(Elem::new_l(tmp));
                idx += new_idx;
            }
            ']' => break,
            ',' => (),
            part => {
                if &line[idx..=idx + 1] == "10" {
                    res.push(Elem::new(10));
                    idx += 1;
                } else {
                    res.push(Elem::new(part as i32 - '0' as i32))
                }
            }
        }
        idx += 1;
    }
    return (res, idx);
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let mut idx = 1;
    let mut sum = 0;
    for mut chunk in &lines.into_iter().chunks(3) {
        let left = chunk.next().unwrap();
        let right = chunk.next().unwrap();
        let a = parse_vec(&left);

        let b = parse_vec(&right);
        println!("== Pair {idx} ==");
        if compare(&a.0, &b.0).1 {
            println!("ok!");
            sum += idx;
        }
        println!("------");
        idx += 1;
    }
    println!("{sum}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let mut elems: Vec<(Vec<Elem>, String)> = Vec::new();

    for mut chunk in &lines.into_iter().chunks(3) {
        let left = chunk.next().unwrap();
        let right = chunk.next().unwrap();
        let a = parse_vec(&left).0;
        let b = parse_vec(&right).0;
        elems.push((a, left));
        elems.push((b, right));
    }

    elems.sort_by(|a, b| {
        if compare(&a.0, &b.0).1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut idx = 1;
    let mut i1 = 0;
    let mut i2 = 0;
    for (_, line) in elems {
        println!("{line}");

        if line == "[[2]]" {
            i1 = idx;
        }
        if line == "[[6]]" {
            i2 = idx;
        }
        idx += 1;
    }
    println!("{i1} {i2} {}", i1 * i2);
}
