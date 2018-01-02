/// Keywords: none

use std::collections::HashMap;
use p033::gcd;

fn is_odd_period(n: u64) -> bool {
    let mut num = 1;

    let root = (n as f64).sqrt() as u64;
    let mut r = root;

    let mut seen = HashMap::new();
    seen.insert((1, 1), 0);

    for step in 1.. {
        let mut fraction = n - (r * r) as u64;

        if fraction == 0 {
            return false;
        }

        let g = gcd(num, fraction);

        if g > 1 {
            fraction = fraction / g;
        }

        let s = seen.entry((r, fraction)).or_insert(step);

        if *s != step {
            return (step - *s) % 2 != 0;
        }

        let rem = (root + r) / fraction;
        r = -(r as i64 - (rem * fraction) as i64) as u64;
        num = fraction;
    }

    panic!("not found");
}

fn run() -> u64 {
    (2..=10000).filter(|v| is_odd_period(*v)).count() as u64
}

problem!{
    tests => [
        q => {run(), "6d62aa4b52071e39f064a930d190b85ab327eb1a5045a8050ac538666ee765ca"},
    ];
}
