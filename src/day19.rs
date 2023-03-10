use rayon::prelude::*;
use regex::Regex;
use std::{cmp::max, collections::HashMap};

#[derive(Debug)]
struct Blueprint {
    orer_ore: i32,
    clayr_ore: i32,
    obsr_ore: i32,
    obsr_clay: i32,
    geoder_ore: i32,
    geoder_obs: i32,
    max_ore: i32,
    total_time: i32,
}

fn solve(
    b: &Blueprint,
    r: (i32, i32, i32, i32),
    i: (i32, i32, i32, i32),
    time: i32,
    cache: &mut HashMap<(u64, i32), i32>,
) -> i32 {
    //rate: (ore, clay, obsidi, geode)

    let key: u64 = ((i.0 as u64 & 0xFF) << 0)
        | ((i.1 as u64 & 0xFF) << 8)
        | ((i.2 as u64 & 0xFF) << 16)
        | ((i.3 as u64 & 0xFF) << 24)
        | ((r.0 as u64 & 0xFF) << 32)
        | ((r.1 as u64 & 0xFF) << 40)
        | ((r.2 as u64 & 0xFF) << 48)
        | ((r.3 as u64 & 0xFF) << 56);
    if let Some(result) = cache.get(&(key, time)) {
        return *result;
    }

    if time == b.total_time - 1 {
        return i.3 + r.3;
    }

    let new_i = (i.0 + r.0, i.1 + r.1, i.2 + r.2, i.3 + r.3);

    let mut cmds: Vec<((i32, i32, i32, i32), (i32, i32, i32, i32))> = Vec::new();

    let need_ore = r.0 < b.max_ore;
    let need_clay = r.1 < b.obsr_clay;
    let need_obs = r.2 < b.geoder_obs;
    // New geode robot?
    let can_build_geode = if i.0 >= b.geoder_ore && i.2 >= b.geoder_obs {
        cmds.push((
            (r.0, r.1, r.2, r.3 + 1),
            (
                i.0 + r.0 - b.geoder_ore,
                i.1 + r.1,
                i.2 + r.2 - b.geoder_obs,
                i.3 + r.3,
            ),
        ));
        true
    } else {
        false
    };

    // New obsidian robot?
    if i.0 >= b.obsr_ore && i.1 >= b.obsr_clay && need_obs && !can_build_geode {
        cmds.push((
            (r.0, r.1, r.2 + 1, r.3),
            (
                i.0 + r.0 - b.obsr_ore,
                i.1 + r.1 - b.obsr_clay,
                i.2 + r.2,
                i.3 + r.3,
            ),
        ));
    }
    // New clay robot?
    if i.0 >= b.clayr_ore && need_clay && !can_build_geode {
        cmds.push((
            (r.0, r.1 + 1, r.2, r.3),
            (i.0 + r.0 - b.clayr_ore, i.1 + r.1, i.2 + r.2, i.3 + r.3),
        ));
    }
    // New ore robot?
    if i.0 >= b.orer_ore && need_ore && !can_build_geode {
        cmds.push((
            (r.0 + 1, r.1, r.2, r.3),
            (i.0 + r.0 - b.orer_ore, i.1 + r.1, i.2 + r.2, i.3 + r.3),
        ));
    }

    // Do nothing?
    if !can_build_geode {
        cmds.push((r, new_i));
    }

    /*
        println!(
            "{}: Updated inventory {:?} (rate {:?}) (cmd l: {})",
            time,
            new_i,
            r,
            cmds.len()
        );
    */
    let mut res = 0;
    for (r, i) in cmds {
        res = max(res, solve(b, r, i, time + 1, cache));
    }

    let entry = cache.entry((key, time)).or_insert(0);
    assert!(*entry == 0);
    *entry = res;

    res
}
fn func(lines: impl Iterator<Item = String>, total_time: i32) {
    let re = Regex::new(
        r"Blueprint ([0-9-]+): Each ore robot costs ([0-9-]+) ore. Each clay robot costs ([0-9-]+) ore. Each obsidian robot costs ([0-9-]+) ore and ([0-9-]+) clay. Each geode robot costs ([0-9-]+) ore and ([0-9-]+) obsidian.",
    )
    .unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for line in lines {
        let caps = re.captures(line.as_str()).unwrap();

        let v = caps
            .iter()
            .filter_map(|x| x.unwrap().as_str().parse().ok())
            .collect::<Vec<i32>>();
        let b = Blueprint {
            orer_ore: v[1],
            clayr_ore: v[2],
            obsr_ore: v[3],
            obsr_clay: v[4],
            geoder_ore: v[5],
            geoder_obs: v[6],
            max_ore: vec![v[1], v[2], v[3], v[5]].into_iter().max().unwrap(),
            total_time,
        };
        dbg!(&b);

        blueprints.push(b);
        //println!("{:?} {:?}", s, b);
    }

    let sum: i32 = blueprints
        .into_par_iter()
        .enumerate()
        .map(|(idx, b)| {
            let res = solve(&b, (1, 0, 0, 0), (0, 0, 0, 0), 0, &mut HashMap::new());
            println!("Res: {idx} {res}");
            (idx as i32 + 1) * res
        })
        .sum();
    println!("{sum}");
}

pub fn part1(lines: impl Iterator<Item = String>) {
    func(lines, 24);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    func(lines, 32);
}
