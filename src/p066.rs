/// Keywords: none

use num::BigUint;

fn continuous_fraction(n: u64) -> Vec<u64> {
    let a0 = (n as f64).sqrt().floor() as u64;

    if a0 * a0 == n {
        return vec![];
    }

    let mut d = 1;
    let mut a = 0;
    let mut m = 0;

    let mut first = true;

    let mut out = vec![];

    while a != 2 * a0 {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;

        if first {
            first = false;
            continue;
        }

        out.push(a);
    }

    out
}

fn result(f: &[u64], p: usize) -> (BigUint, BigUint) {
    if p == 2 {
        let a: BigUint = f[0].into();
        let b: BigUint = f[1].into();
        let one: BigUint = 1u32.into();
        return (a * b.clone() + one, b);
    }

    let (rnum, rden) = result(&f[1..], p - 1);
    (f[0] * rnum.clone() + rden, rnum)
}

fn run() -> String {
    let mut max = 0u64.into();

    for d in 0..=1000 {
        let fractions = continuous_fraction(d);

        // println!("{} = {:?}", d, fractions);

        if fractions.len() < 2 {
            continue;
        }

        let len = fractions.len() - 1;

        let (x, y) = if len % 2 == 0 {
            result(&fractions, len)
        } else {
            let mut full = fractions.clone();
            full.extend(fractions[1..].iter().cloned());
            full.extend(fractions[1..].iter().cloned());
            result(&full, len * 2 + 1)
        };

        println!("{}: x = {}, y = {}", d, x, y);

        max = if x > max {
            x
        } else { max };
    }

    max.to_string()
}

problem!{
    tests => [
        q => (run(), "".to_string()),
    ];
}
