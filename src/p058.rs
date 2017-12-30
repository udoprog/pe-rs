/// Keywords: none

use prime::is_prime;

fn run() -> u64 {
    let mut primes = 0u64;
    let mut total = 1u64;

    for i in (3u64..).step_by(2) {
        let base = i.pow(2);
        let layer_nums: Vec<_> = (1..4).map(|n| base - (n * (i - 1))).collect();
        let layer_primes = layer_nums.iter().cloned().filter(|n| is_prime(*n)).count() as u64;

        primes += layer_primes;
        total += 4;

        if primes * 10 <= total {
            return i;
        }
    }

    panic!("no layer");
}

problem!{
    tests => [
        q => {run(), "196f327021627b6a48db9c6e0a3388d110909d4bb957eb3fbc90ff1ecbda42cb"},
    ];
}
