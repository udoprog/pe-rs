/// Keywords: none

use sieve::FixedSieve;
use std::collections::HashSet;

fn run() -> u64 {
    let primes: Vec<u64> = FixedSieve::new(1_000_000u64).collect();
    let lookup: HashSet<u64> = primes.iter().cloned().collect();

    let mut found = None;
    let mut current = 0;

    for end in 2..primes.len() {
        for start in current..(end - 1) {
            let sum: u64 = primes[start..end].iter().cloned().sum();

            if lookup.contains(&sum) {
                found = Some(found.map(|v| u64::max(v, sum)).unwrap_or(sum));
            }

            if sum > 1_000_000u64 {
                current = start - 1;
                break;
            }
        }
    }

    found.expect("no result")
}

problem!{
    tests => [
        q => {run(), "6ee74ef623df9fb69facd30b91ed78fe70370462bb267097f0dfeef9d9b057bb"},
    ];
}
