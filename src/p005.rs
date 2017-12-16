/// Keywords: prime

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
        q => {run(20), "1ba90ab11bfb2d2400545337212b0de2a5c7f399215175ade6396e91388912b1"},
    ];
}
