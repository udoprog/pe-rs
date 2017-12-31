/// Keywords: none

use sieve::Sieve;
use prime::is_prime;
use digits::digits;
use std::collections::HashMap;

fn run() -> u64 {
    let mut sieve = Sieve::infinite::<u64>();
    let mut primes = Vec::new();
    let mut pairs: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut order = Vec::new();

    loop {
        let index = primes.len();
        primes.extend((&mut sieve).take(1000));

        // backfill added primes
        if index > 0 {
            for a in 0..primes.len() {
                for b in index..primes.len() {
                    let a = primes[a];
                    let b = primes[b];

                    add_pairs(&mut pairs, &mut order, a, b);
                }
            }
        }

        for a in index..primes.len() {
            for b in a..primes.len() {
                let a = primes[a];
                let b = primes[b];

                add_pairs(&mut pairs, &mut order, a, b);
            }
        }

        let mut min = None;

        for (a, v) in pairs.iter() {
            let mut values = v.clone();
            values.push(*a);

            let mut index = 0;

            while index < values.len() {
                let b = values[index];
                let paired = pairs.get(&b).expect("no neigh");
                values.drain_filter(|i| !paired.contains(i) && *i != b).count();
                index += 1;
            }

            if values.len() == 5 {
                let sum = values.iter().cloned().sum::<u64>();
                min = Some(min.map(|m| u64::min(sum, m)).unwrap_or(sum));
            }
        }

        if let Some(m) = min {
            return m;
        }
    }

    fn add_pairs(pairs: &mut HashMap<u64, Vec<u64>>, order: &mut Vec<u64>, a: u64, b: u64) {
        let a_digs = digits(a).collect::<Vec<_>>();
        let b_digs = digits(b).collect::<Vec<_>>();

        let next_len = a_digs.len() + b_digs.len();
        let len = order.len();

        if next_len > len {
            order.extend((len..next_len).map(|v| 10u64.pow(v as u32)));
        }

        let a_sum = {
            let mut test = a_digs.clone();
            test.extend(b_digs.clone());
            test.into_iter().zip(order.iter().cloned()).map(|(a, b)| a as u64 * b).sum::<u64>()
        };

        let b_sum = {
            let mut test = b_digs.clone();
            test.extend(a_digs.clone());
            test.into_iter().zip(order.iter().cloned()).map(|(a, b)| a as u64 * b).sum::<u64>()
        };

        if is_prime(a_sum) && is_prime(b_sum) {
            pairs.entry(a).or_insert_with(Vec::new).push(b);
            pairs.entry(b).or_insert_with(Vec::new).push(a);
        }
    }
}

problem!{
    tests => [
        q => {run(), "ad7c26db722221bfb1bf7e3c36b501bedf8be857b1cfa8664fccb074b54354f9"},
    ];
}
