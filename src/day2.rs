use std::io::prelude::*;

fn score(data: Vec<&str>) -> i32 {
    let a = data[0];
    let b = data[1];

    let (winner, new_shape) = match (a, b) {
        ("A", "X") => (0, "Z"), // rock v scissor
        ("A", "Y") => (3, "X"), // rock v rock
        ("A", "Z") => (6, "Y"), // rock v paper

        ("B", "X") => (0, "X"), // paper v rock
        ("B", "Y") => (3, "Y"), // paper v paper
        ("B", "Z") => (6, "Z"), // paper v scissor

        ("C", "X") => (0, "Y"), // scissor v paper
        ("C", "Y") => (3, "Z"), // scissor v scissor
        ("C", "Z") => (6, "X"), // scissor v rock
        (_, _) => (0, "_"),
    };

    let shape = match new_shape {
        "X" => 1, // rock
        "Y" => 2, // paper
        "Z" => 3, // scissor
        _ => 0,
    };

    return winner + shape;
}

pub fn day2() {
    let res: i32 = std::io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|elt| score(elt.split_whitespace().collect()))
        .sum();

    println!("{res}");
}
