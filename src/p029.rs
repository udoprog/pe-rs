/// Keywords: spiral memory

use std::collections::HashSet;
use sieve::Sieve;
use num_bigint::BigUint;
use num_traits::pow::pow;

/// Here to verify results.
fn bignum<A, B>(a: A, b: B) -> u32
where
    A: IntoIterator<Item = u32>,
    B: IntoIterator<Item = u32>,
{
    let mut set = HashSet::new();

    let a: Vec<BigUint> = a.into_iter().map(Into::into).collect();
    let b: Vec<usize> = b.into_iter().map(|v| v as usize).collect();

    for a in a.into_iter() {
        for b in b.iter().cloned() {
            set.insert(pow(a.clone(), b));
        }
    }

    set.len() as u32
}

/// No bignum
fn run<A, B>(a: A, b: B) -> u32
where
    A: IntoIterator<Item = u32>,
    B: IntoIterator<Item = u32>,
{
    let a: Vec<u32> = a.into_iter().collect();
    let b: Vec<u32> = b.into_iter().collect();

    let base_max = a.iter().cloned().max().expect("no max");

    // collect all base primes.
    let primes: Vec<u32> = Sieve::bounded(base_max + 1).collect();

    let mut set: HashSet<Vec<(u32, u32)>> = HashSet::new();

    for mut num in a.iter().cloned() {
        let mut factors = Vec::new();

        for p in primes.iter().cloned() {
            let mut e = 0;

            while num % p == 0 {
                num = num / p;
                e += 1;
            }

            if e > 0 {
                factors.push((p, e));
            }

            if num == 1 {
                break;
            }
        }

        for b in b.iter().cloned() {
            set.insert(factors.iter().cloned().map(|(p, e)| (p, e * b)).collect());
        }
    }

    set.len() as u32
}

problem!{
    tests => [
        example1 => (run(2..=5, 2..=5), 15),
        brute_q => {bignum(2..=100, 2..=100), "a207c35d8417aeed4c9e78bcf83f936cd8191c702893be62aa690ce16bc909ca"},
        q => {run(2..=100, 2..=100), "a207c35d8417aeed4c9e78bcf83f936cd8191c702893be62aa690ce16bc909ca"},
    ];
}
