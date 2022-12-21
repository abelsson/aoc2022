use itertools::Itertools;

struct Dir {
    name: String,
    path: String,
    size: i32,
    subdirs: Vec<Dir>,
}

fn walk(dir: &Dir, sizes: &mut Vec<i32>) -> i32 {
    let mut total = dir.size;

    for sub in dir.subdirs.iter() {
        total += walk(&sub, sizes);
    }

    println!("{} size {total}", dir.name);
    sizes.push(total);
    return total;
}

fn find<'a>(dir: &'a mut Dir, path: &str) -> Option<&'a mut Dir> {
    if dir.path == path {
        return Some(dir);
    }

    for sub in dir.subdirs.iter_mut() {
        if let Some(result) = find(sub, path) {
            return Some(result);
        }
    }

    return None;
}

pub fn func(lines: impl Iterator<Item = String>) {
    let mut root = Dir {
        name: "/".to_owned(),
        path: "".to_owned(),
        size: 0,
        subdirs: Vec::<Dir>::new(),
    };

    let mut cur = &mut root;

    for line in lines {
        let mut x = line.split_whitespace();
        match x.next() {
            Some("$") => match x.next() {
                Some("cd") => {
                    let dest = x.next().unwrap();
                    println!("cd {dest}");
                    if dest == "/" {
                        cur = &mut root;
                        continue;
                    }
                    if dest == ".." {
                        let foo: Vec<&str> = cur.path.split("/").collect();
                        let len = foo.len();
                        let bar = foo[..len - 1].join("/");
                        println!("new dir {bar}");
                        cur = find(&mut root, &bar).unwrap();
                        continue;
                    }

                    match cur.subdirs.iter().position(|x| x.name == dest) {
                        Some(i) => cur = &mut cur.subdirs[i],
                        None => {
                            let dir = Dir {
                                name: dest.to_owned(),
                                path: format!("{}/{}", cur.path, dest),
                                size: 0,
                                subdirs: Vec::<Dir>::new(),
                            };
                            cur.subdirs.push(dir);
                            let len = cur.subdirs.len();
                            cur = &mut cur.subdirs[len - 1];
                        }
                    }
                }
                Some("ls") => println!("ls"),
                _ => todo!(),
            },
            Some("dir") => (),
            Some(string) => {
                let size: i32 = string.parse().unwrap();
                cur.size += size;
                println!("Size: {} {size} (total {})", cur.path, cur.size);
            }
            _ => todo!(),
        }
    }

    println!("------------");
    let mut sizes = Vec::<i32>::new();
    walk(&root, &mut sizes);

    let sum: i32 = sizes.iter().filter(|x| **x <= 100000).sum();
    println!("Part 1: Sum: {sum}");

    let available = 70000000 - 41272621;
    let needed = 30000000 - available;
    println!("Needed {needed}");
    let smallest: i32 = sizes
        .iter()
        .filter(|x| **x >= needed)
        .sorted()
        .take(1)
        .sum();
    println!("Part 2: Smallest to delete: {smallest}");
}
