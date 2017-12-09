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
        example => (2520, run(10)),
        q => (232792560, run(20)),
    ];
}
