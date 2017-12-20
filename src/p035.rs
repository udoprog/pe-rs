/// Keywords: primes, rotate

use sieve::FixedSieve;
use p032::digits;

fn run(limit: usize) -> u64 {
    let mut primes = vec![false; limit];
    let mut test = Vec::new();

    let mut count = 0u64;

    for p in FixedSieve::new(limit as u64) {
        primes[p as usize] = true;
        test.push(p);
    }

    for p in test.into_iter() {
        let digs: Vec<u8> = digits(p).collect();

        let mut all_primes = true;

        for i in 1..digs.len() {
            let (first, rest) = digs.split_at(i);

            let r: u64 = rest.iter()
                .chain(first)
                .zip((0..).map(|n| 10u64.pow(n)))
                .map(|(a, b)| *a as u64 * b)
                .sum();

            if !primes[r as usize] {
                all_primes = false;
                break;
            }
        }

        if all_primes {
            count += 1;
        }
    }

    count
}

problem!{
    tests => [
        example1 => (run(100), 13),
        q => {run(1_000_000), "02d20bbd7e394ad5999a4cebabac9619732c343a4cac99470c03e23ba2bdc2bc"},
    ];
}
