use std::collections::{HashMap, HashSet, VecDeque};

fn possible(map: &Vec<Vec<i32>>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = pos;
    let mut elev = map[y as usize][x as usize];
    let mut c: Vec<(i32, i32)> = Vec::new();
    if elev == -14 {
	elev = 0;
    }
    if ((x + 1) as usize) < map[0].len() && (map[y as usize][(x + 1) as usize] - elev) <= 1 {
	c.push((x + 1, y));
    }

    if x - 1 >= 0 && (map[y as usize][(x - 1) as usize] - elev) <= 1 {
	c.push((x - 1, y));
    }

    if y - 1 >= 0 && (map[(y - 1) as usize][x as usize] - elev) <= 1 {
	c.push((x, y - 1));
    }

    if ((y + 1) as usize) < map.len() && (map[(y + 1) as usize][x as usize] - elev) <= 1 {
	c.push((x, y + 1));
    }

    return c;
}

fn print(map: &Vec<Vec<i32>>, visited: &Vec<(i32, i32)>) {
    let mut x = 0;
    let mut y = 0;
    for row in map {
	x = 0;
	for pos in row {
	    if visited.contains(&(x, y)) {
		print!(" x ");
	    } else {
		print!("{pos:2} ");
	    }
	    x += 1;
	}
	y += 1;
	println!("");
    }
    println!("");
}

fn bfs(
    map: &Vec<Vec<i32>>,
    start: &(i32, i32),
    target: &(i32, i32)
) -> Option<Vec<(i32, i32)>> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut come_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start.0, start.1));

    while let Some(current) = queue.pop_front() {

	if current == *target {

	    let mut pos = come_from.get(&target);
	    let mut len = 0;
	    let mut h: Vec<(i32, i32)> = Vec::new();
	    while pos.unwrap() != start {
		h.push(*pos.unwrap());
		pos = come_from.get(&pos.unwrap());
		len += 1;
	    }
	    println!("{}", len + 1);
	}

	let cands = possible(map, current);
	for cand in cands {
	    if !visited.contains(&cand) {
		visited.insert(cand);
		queue.push_back(cand);
		come_from.insert(cand, current);
	    }
	}
    }

    None
}

pub fn func(lines: impl Iterator<Item = String>, starting_values: &Vec<i32>) {
    let mut map: Vec<Vec<i32>> = Vec::new();
    for line in lines {
	map.push(
	    line.chars()
		.map(|ch| ch as i32 - 'a' as i32)
		.collect::<Vec<i32>>(),
	);
    }

    let mut target: (i32, i32) = (0, 0);
    let mut x = 0;
    let mut y = 0;

    //let res : Vec<usize> = map.iter().flatten().enumerate().filter(|x| *x.1 == -28).map(|x| x.0).collect();
    //dbg!(res[0] / map[0].len());
    for row in &map {
	x = 0;
	for pos in row {
	    if *pos == -28 {
		target = (x, y);
		print!(" T ");
	    } else {
		print!("{pos:2} ");
	    }
	    x += 1;
	}
	y += 1;
	println!("");
    }

    map[target.1 as usize][target.0 as usize] = 'z' as i32 - 'a' as i32;

    y = 0;
    for row in &map {
	x = 0;
	for pos in row {
	    if starting_values.contains(pos) {
		bfs(&map, &(x, y), &target);
	    }
	    x += 1;
	}
	y += 1;
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, &vec![-14]);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, &vec![-14, 0]);
}
