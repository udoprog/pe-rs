use sieve::Sieve;

fn run(upper: u64) -> u64 {
    let mut result = 1u64;

    for p in Sieve::new(upper) {
        let mut v = p;

        while v * p < upper {
            v *= p;
        }

        result *= v;
    }

    result
}

problem!{
    tests => [
        example => (run(10), 2520),
        q => (run(20), 232792560),
    ];
}
