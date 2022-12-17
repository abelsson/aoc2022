use itertools::Itertools;
use regex::Regex;
use rayon::prelude::*;
use std::iter::zip;

fn manhattan_dist(a : &(i32, i32), b: &(i32, i32)) -> i32 {
    let (x1, x2) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
    let (y1, y2) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
    return x2 - x1 + y2 - y1;
}

fn func(lines: impl Iterator<Item = String>, part2: bool) {

    let mut sensors : Vec<(i32, i32)> = Vec::new();
    let mut beacons : Vec<(i32, i32)> = Vec::new();

    let re = Regex::new(r"Sensor at x=([0-9-]+), y=([0-9-]+): closest beacon is at x=([0-9-]+), y=([0-9-]+)").unwrap();
    for line in lines {

	let caps = re.captures(line.as_str()).unwrap();

	let mut foo = caps.iter().filter_map(|x| x.unwrap().as_str().parse().ok());
	let s = foo.next_tuple::<(i32, i32)>().unwrap();
	let b = foo.next_tuple::<(i32, i32)>().unwrap();
	sensors.push(s);
	beacons.push(b);
	println!("{:?} {:?}", s, b);

    }

    let min_dists : Vec<i32> = sensors.iter().map(|s| beacons.iter().map(|b| manhattan_dist(s, b)).min()).flatten().collect();

    let mut res = 0;
    for x in -20000000..20000000 {
	let coord = (x, 2000000);

	let mut idx = 0;
	let mut can_have_beacon = true;

	for s in &sensors {
	    let dist = manhattan_dist(&coord, s);
	    if dist <= min_dists[idx] {
		can_have_beacon = false;
	    }
	    /*
	    println!("{dist} to sensor {:?}, coverage radius for sensor: {} : {}",
		     sensors[idx], min_dists[idx],
		      dist <= min_dists[idx]);
	     */
	    idx+=1;
	}
	//println!("{x}: {can_have_beacon}");
	if !can_have_beacon && !beacons.contains(&coord) && !sensors.contains(&coord) {
	    res += 1;
	}
    }

    println!("{res}");
}


pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, false);
}

pub fn part2(lines: impl Iterator<Item = String>) {

    let mut sensors : Vec<(i32, i32)> = Vec::new();
    let mut beacons : Vec<(i32, i32)> = Vec::new();

    let re = Regex::new(r"Sensor at x=([0-9-]+), y=([0-9-]+): closest beacon is at x=([0-9-]+), y=([0-9-]+)").unwrap();
    for line in lines {

	let caps = re.captures(line.as_str()).unwrap();

	let mut foo = caps.iter().filter_map(|x| x.unwrap().as_str().parse().ok());
	let s = foo.next_tuple::<(i32, i32)>().unwrap();
	let b = foo.next_tuple::<(i32, i32)>().unwrap();
	sensors.push(s);
	beacons.push(b);
	println!("{:?} {:?}", s, b);

    }

    let min_dists : Vec<i32> = sensors.iter().map(|s| beacons.iter().map(|b| manhattan_dist(s, b)).min()).flatten().collect();

    let area = 4_000_000;
    for y in 0..=area {
	//println!("Considering {y}");
	let mut xcand : Vec<i32> = Vec::new();
	for (s, radius) in zip(&sensors, &min_dists) {
	    let ydist = *radius - (s.1 - y).abs();

	    if ydist >= 0 {
		let start_x = s.0 - ydist - 1;
		let end_x = s.0 + ydist + 1;
		//println!("{y}: {ydist} {start_x} {end_x}");
		if start_x >= 0 && start_x <= area {
		    xcand.push(start_x);
		}
		if end_x >= 0 && end_x <= area {
		    xcand.push(end_x);
		}
	    }
	}
	for x in xcand {
	    let coord = (x, y);

	    let mut idx = 0;
	    let mut can_have_beacon = true;
	    for s in &sensors {
		let dist = manhattan_dist(&coord, s);
		if dist <= min_dists[idx] {
		    can_have_beacon = false;
		    break;
		}
		idx+=1;
	    }
	    if can_have_beacon {
		println!("Can have beacon at {:?}", coord);
		return;
	    }
	}
    }
}
