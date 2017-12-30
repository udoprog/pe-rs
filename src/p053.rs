/// Keywords: factorials

use p033::gcd;

fn run() -> u64 {
    let mut count = 0;

    for n in 1u64..=100 {
        for r in 1u64..=n {
            if calc(n, r, 1_000_000) {
                count += 1;
            }
        }
    }

    count
}

fn calc(n: u64, r: u64, limit: u64) -> bool {
    let mut num = vec![];

    for n in (n - r + 1)..=n {
        num.push(n);
    }

    for mut d in 1..=r {
        for n in num.iter_mut() {
            let g = gcd(*n, d);

            if g > 1 {
                *n = *n / g;
                d = d / g;
            }

            if *n % d == 0 {
                *n = *n / d;
                break;
            }
        }

        if d != 1 {
            panic!("could not simplify denominator");
        }
    }

    let mut r = 1u64;

    for n in num {
        r *= n;

        if r > limit {
            return true;
        }
    }

    false
}

problem!{
    tests => [
        q => {run(), "9705cc6128a60cc22581217b715750a6053b2ddda67cc3af7e14803b27cf0c1f"},
    ];
}
