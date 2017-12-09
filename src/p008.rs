static INPUT: &str = include_str!("p008_input.txt");

use std::io::{BufRead, BufReader, Cursor};

fn run(count: usize) -> u64 {
    let mut input: Vec<u64> = Vec::new();

    for line in BufReader::new(Cursor::new(INPUT)).lines() {
        let line = line.expect("bad line");
        let line = line.trim();

        for c in line.chars() {
            input.push(c.to_digit(10).expect("bad number") as u64);
        }
    }

    let mut products = Vec::new();

    for i in 0..(input.len() - (count - 1)) {
        products.push(input[i..(i + count)].iter().fold(1u64, |r, &v| r * v))
    }

    products.sort();
    products[products.len() - 1]
}

problem!{
    tests => [
        example => (run(4), 5832),
        q => {run(13), "b9fb30b6553415e9150051ce5710a93d0f55b22557c0068d8e16619a388f145a"},
    ];
}
