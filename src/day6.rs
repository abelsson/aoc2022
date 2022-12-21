use std::collections::HashSet;

pub fn func(mut lines: impl Iterator<Item = String>, n: usize) {
    let line = lines.next().unwrap();

    let mut idx = 0;
    for chars in line.as_bytes()[0..].windows(n) {
        let mut uniq = HashSet::new();
        let res = chars.into_iter().all(|x| uniq.insert(x));
        if res {
            println!("{}", idx + n);
            break;
        }
        idx += 1;
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 4);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, 14);
}
