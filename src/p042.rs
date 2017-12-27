/// Keywords: none

use input::parse;
use std::io::{Cursor, Read};

fn run<R: Read>(reader: R) -> u64 {
    let mut count = 0;

    for word in parse(reader) {
        let word = word.expect("bad word");

        let mut sum = 0u64;

        for c in word.chars() {
            sum += (c as u8 - 'A' as u8) as u64 + 1;
        }

        let solved = solve(sum as f64);

        if triangle(solved.round() as u64) == sum {
            count += 1;
        }
    }

    return count;

    fn triangle(n: u64) -> u64 {
        (n * (n + 1)) / 2
    }

    fn solve(t: f64) -> f64 {
        -0.5f64 + (0.25f64 + 2f64 * t).sqrt()
    }
}

problem!{
    tests => [
        q => {run(Cursor::new(include_str!("p042_words.txt"))), "79d6eaa2676189eb927f2e16a70091474078e2117c3fc607d35cdc6b591ef355"},
    ];
}
