/// Keywords: primes

use std::collections::{VecDeque, HashSet};
use prime::is_prime;

fn run() -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back((10, 2));
    queue.push_back((10, 3));
    queue.push_back((10, 5));
    queue.push_back((10, 7));

    let mut sum = 0;
    let mut found = HashSet::new();

    while let Some((order, n)) = queue.pop_front() {
        if n >= 10 && is_right_truncatable(n, order) {
            if found.insert(n) {
                sum += n;
            }
        }

        for d in 0..10 {
            let test = (n * 10) + d;

            if test == 0 || is_prime(test) {
                queue.push_back((order * 10, test));
            }
        }
    }

    return sum;

    fn is_right_truncatable(n: u64, mut order: u64) -> bool {
        while order > 0 {
            let test = n % order;

            if !(test == 0 || is_prime(test)) {
                return false;
            }

            order = order / 10;
        }

        true
    }
}

problem!{
    tests => [
        q => {run(), "e9800abda89919edac504e90dac91f95e0778e3ba0f21a0bac4e77a84766eaaf"},
    ];
}
