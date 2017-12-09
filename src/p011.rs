static INPUT: &str = include_str!("p011_grid.txt");

use std::io::{BufRead, BufReader, Cursor};
use std::iter;

fn run(count: usize) -> u64 {
    let mut input: Vec<Vec<u64>> = Vec::new();

    for line in BufReader::new(Cursor::new(INPUT)).lines() {
        let line = line.expect("bad line");
        let line = line.trim();

        input.push(
            line.split_whitespace()
                .map(|d| d.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .expect("a bad line"),
        );
    }

    let width = input.iter().map(|line| line.len()).min().expect(
        "bad width",
    );

    let mut products = Vec::new();

    for line in &input {
        // horizontally
        for i in 0..(line.len() - (count - 1)) {
            products.push(line[i..(i + count)].iter().fold(1u64, |r, &v| r * v))
        }
    }

    // vertically
    for x in 0..width {
        for y in 0..(input.len() - (count - 1)) {
            products.push(product(&input, iter::repeat(x).take(4).zip(y..=y+3)));
        }
    }

    // diagonals
    for x in 0..(width - (count - 1)) {
        for y in 0..(input.len() - (count - 1)) {
            products.push(product(&input, (x..=x+3).zip(y..=y+3)));
            products.push(product(&input, (x..=x+3).rev().zip(y..=y+3)));
        }
    }

    products.sort();
    return products[products.len() - 1];

    fn product<I>(input: &Vec<Vec<u64>>, range: I) -> u64
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        range.into_iter().map(|(x, y)| input[y][x]).fold(1u64, |p, v| p * v)
    }
}

problem!{
    tests => [
        q => {run(4), "9ded5bc849d33e477aa9c944138d34f0aacc485a372e84464e8a572712a5b7da"},
    ];
}
