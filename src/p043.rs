/// Keywords: none

use p024::Permutations;

// TODO: solution is a bit slow. make faster by generating legal triplets for each number and
// building permutations from them (bottom-up).
fn run() -> u64 {
    let primes = vec![2, 3, 5, 7, 11, 13, 17];
    let orders: Vec<u64> = (0..3).rev().map(|a| 10u64.pow(a)).collect();
    let digits: Vec<u64> = (0u64..10).collect();
    let ord10: Vec<u64> = (0..10).rev().map(|a| 10u64.pow(a)).collect();

    let mut sum = 0;

    for digits in Permutations::new(digits) {
        let mut all = true;

        for (s, p) in (1..8).zip(primes.iter().cloned()) {
            let sum: u64 = digits[s..(s + 3)].iter().zip(orders.iter().cloned()).map(|(a, b)| a * b).sum();

            if sum % p != 0 {
                all = false;
            }
        }

        if all {
            sum += digits.iter().cloned().zip(ord10.iter().cloned()).map(|(a, b)| a * b).sum::<u64>();
        }
    }

    sum
}

problem!{
    tests => [
        q => {run(), "6512f20c244844b6130204379601855098826afa1b55ff91c293c853ddf67db5"},
    ];
}
