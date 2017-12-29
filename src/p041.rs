/// Keywords: primes

use p024::Permutations;
use prime::is_prime;

fn run() -> u64 {
    for n in (1..10).rev() {
        let orders: Vec<u64> = (0..n).rev().map(|n| 10u64.pow(n as u32)).collect();
        let digits: Vec<u64> = (1u64..=n).collect();

        let mut largest = None;

        for digits in Permutations::new(digits) {
            let sum: u64 = digits.into_iter().zip(orders.iter().cloned()).map(|(a, b)| a * b).sum();

            if is_prime(sum) {
                largest = Some(largest.map(|old| u64::max(old, sum)).unwrap_or(sum));
            }
        }

        if let Some(largest) = largest {
            return largest;
        }
    }

    panic!("no prime");
}

problem!{
    tests => [
        q => {run(), "bf05020e70de94e26dba112bb6fb7b0755db5ca88c7225e99187c5a08c8a0428"},
    ];
}
