/// Keywords: none

use p032::digits;

/// Simple GCD implementation.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if a == b {
            return a;
        }

        if a == 1 || b == 1 {
            return 1;
        }

        if a > b {
            a -= b;
        }

        if b > a {
            b -= a;
        }
    }
}

fn run() -> u64 {
    let mut nom = 1;
    let mut den = 1;

    for n in (10u64..100).filter(|d| d % 10 != 0) {
        for d in ((n + 1)..100).filter(|d| d % 10 != 0) {
            let g = gcd(n, d);

            if g == 1 {
                continue;
            }

            let n_digits: Vec<u8> = digits(n).collect();
            let d_digits: Vec<u8> = digits(d).collect();

            for c in n_digits.iter().cloned().filter(|n| d_digits.contains(n)) {
                let sn = n_digits.iter().cloned().filter(|n| *n != c).next().unwrap_or(c) as u64;
                let sd = d_digits.iter().cloned().filter(|n| *n != c).next().unwrap_or(c) as u64;

                let sg = gcd(sn, sd);

                if (sn / sg, sd / sg) == (n / g, d / g) {
                    nom *= n;
                    den *= d;
                }
            }
        }
    }

    den / gcd(nom, den)
}

problem!{
    tests => [
        gcd_1 => (gcd(54, 24), 6),
        gcd_2 => (gcd(10, 20), 10),
        gcd_3 => (gcd(12345678, 20), 2),
        gcd_4 => (gcd(12345678, 1), 1),
        q => {run(), "ad57366865126e55649ecb23ae1d48887544976efea46a48eb5d85a6eeb4d306"},
    ];
}
