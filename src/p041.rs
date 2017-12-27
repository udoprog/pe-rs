/// Keywords: primes

use p024::Permutations;

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

    fn is_prime(n: u64) -> bool {
        if n == 1 {
            return false;
        }

        let ceil = (n as f64).sqrt().ceil() as u64;

        for i in 2..=ceil {
            if n % i == 0 {
                return false;
            }
        }

        true
    }
}

problem!{
    tests => [
        q => {run(), "bf05020e70de94e26dba112bb6fb7b0755db5ca88c7225e99187c5a08c8a0428"},
    ];
}
