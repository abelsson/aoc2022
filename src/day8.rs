fn visible(nums: &Vec<Vec<u32>>, tx: usize, ty: usize) -> bool {
    let w = nums[0].len();
    let h = nums.len();

    let height = nums[ty][tx];
    let mut visible = vec![true, true, true, true];

    if tx == 0 || tx == w - 1 || ty == 0 || ty == h - 1 {
        return true;
    }
    // check row
    for x in 0..tx {
        if nums[ty][x] >= height {
            //println!("nums[{ty}][{x}] = {} >= {height}", nums[ty][x]);
            visible[0] = false;
        }
    }
    for x in tx + 1..w {
        if nums[ty][x] >= height {
            //println!("nums[{ty}][{x}] = {} >= {height}", nums[ty][x]);
            visible[1] = false;
        }
    }
    // check column
    for y in 0..ty {
        if nums[y][tx] >= height {
            visible[2] = false;
        }
    }
    for y in ty + 1..h {
        if nums[y][tx] >= height {
            visible[3] = false;
        }
    }
    return visible.iter().any(|x| *x);
}

fn scenic_score(nums: &Vec<Vec<u32>>, tx: usize, ty: usize) -> u32 {
    let w = nums[0].len();
    let h = nums.len();

    let height = nums[ty][tx];
    let mut score = vec![0, 0, 0, 0];
    // check left
    //score[0] = (0..tx).rev().take_while(|x| nums[ty][x] < height).len();

    //score[0] = (0..tx).rev().take_while(|x| nums[ty][*x] < height).count() as u32;
    //score[1] = (tx + 1..w).take_while(|x| nums[ty][*x] < height).count() as u32;
    //score[2] = (0..ty).rev().take_while(|y| nums[*y][tx] < height).count() as u32;
    //score[3] = (ty + 1..h).take_while(|y| nums[*y][tx] < height).count() as u32;

    //for foo in (0..tx).rev().take_while(|x| nums[ty][x] <= height) {
    //println!("{foo}: nums[{ty}][{x}] = {} >= {height}", nums[ty][foo]);
    //}
    for x in (0..tx).rev() {
        score[0] += 1;
        if nums[ty][x] >= height {
            break;
        }
    }
    // check right
    for x in tx + 1..w {
        score[1] += 1;

        if nums[ty][x] >= height {
            break;
        }
    }
    // check up
    for y in (0..ty).rev() {
        score[2] += 1;
        if nums[y][tx] >= height {
            break;
        }
    }
    // check down
    for y in ty + 1..h {
        score[3] += 1;

        if nums[y][tx] >= height {
            break;
        }
    }

    return score.iter().product();
}

pub fn func(lines: impl Iterator<Item = String>) {
    let nums = lines
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let s = scenic_score(&nums, 2, 3);
    //return;
    let mut count = 0;
    let mut scores = vec![0];
    for (y, row) in nums.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let vis = visible(&nums, x, y);
            let score = scenic_score(&nums, x, y);
            scores.push(score);
            if vis {
                print!("x");
                count += 1;
            } else {
                print!("-");
            }
        }
        println!();
    }
    let max_score = scores.iter().max().unwrap();
    println!("Part 1: {count}\nPart 2: {max_score}");
}
