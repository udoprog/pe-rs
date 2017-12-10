use std::mem;
use std::collections::HashSet;

use p021::proper_divisors;

use self::Class::*;

/// An iterator over all abundant numbers.
struct Abundant {
    n: u64,
}

impl Abundant {
    pub fn new() -> Abundant {
        Abundant {
            n: 1
        }
    }
}

impl Iterator for Abundant {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.n + 1;
            let value = mem::replace(&mut self.n, next);

            if let Abundant = classify(value) {
                return Some(value);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Class {
    Deficient,
    Perfect,
    Abundant,
}

fn classify(n: u64) -> Class {
    match proper_divisors(n).sum::<u64>() {
        v if v < n => Deficient,
        v if v > n => Abundant,
        _ => Perfect,
    }
}

// TODO: Make faster.
fn run() -> u64 {
    let ceil = 28123u64;
    let abundant = Abundant::new().take_while(|n| *n < ceil).collect::<Vec<_>>();

    let mut known: HashSet<u64> = HashSet::new();

    // building a set of known number is _slow_
    for (s, bn) in abundant.iter().enumerate() {
        for num in abundant.iter().skip(s).map(|d| (*d, bn, bn + *d)).take_while(|d| d.2 <= ceil) {
            known.insert(num.2);
        }
    }

    let mut sum = 0;

    'outer: for n in 1..=ceil {
        if known.contains(&n) {
            continue;
        }

        sum += n;
    }

    sum
}

problem!{
    tests => [
        classify_perfect => (classify(28), Perfect),
        classify_deficient => (classify(27), Deficient),
        classify_abundant => (classify(12), Abundant),
        q => {run(), "42e2552a2f589e021824339e2508629ffa00b3489ea467f47e77a1ea97e735c9"},
    ];
}
