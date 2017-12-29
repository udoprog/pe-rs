/// Keywords: none

use sieve::Sieve;
use comp_sieve::CompSieve;

fn run() -> u64 {
    for c in CompSieve::infinite::<u64>() {
        if c % 2 == 0 {
            continue;
        }

        let mut any = false;

        for p in Sieve::bounded(c) {
            let mut rest = c - p;

            // not divisible by two
            if rest % 2 != 0 {
                continue;
            }

            rest = rest / 2;

            let sqrt = (rest as f64).sqrt();

            if sqrt.ceil() != sqrt {
                continue;
            }

            any = true;
            break;
        }

        if !any {
            return c;
        }
    }

    return 0;
}

problem!{
    tests => [
        q => {run(), "8485ee802cc628b8cbd82476133d11b57af87e00711516a703525a9af0193b12"},
    ];
}
