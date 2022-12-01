use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    let mut v = Vec::new();
    let mut sum = 0;
    for maybeline in stdin.lock().lines() {
	let line = maybeline.unwrap();
	if line.is_empty() {
	    v.push(sum);
	    sum = 0;
	} else {
	    sum += line.parse::<i64>().unwrap();
	}
    }
    v.push(sum);
    v.sort();
    let result = match v.len() {
	0 => None,
	n => Some(v[n-1] + v[n-2] + v[n-3])
    };
    println!("{}", result.unwrap());
}
