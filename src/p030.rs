/// Keywords: permutations

use std::iter;

fn numperm<C>(size: usize, mut callback: C)
where
    C: FnMut(&[u64]) -> (),
{
    let mut values = vec![1u64; size];
    callback(&values);

    loop {
        let mut nth = 0;
        let mut carry = 1;

        while carry > 0 && nth < values.len() {
            let v = values[nth] + carry;
            values[nth] = v % 10;
            carry = v / 10;
            nth += 1;
        }

        // overflow!
        if carry > 0 {
            break;
        }

        callback(&values);
    }
}

/// NOTE: probably more efficient to use the calculate upper bound and check each number until it.
/// But permutations are more fun and this problem is constrained enough so I'm taking that route.
fn run(power: u32) -> u64 {
    let mut results: Vec<u64> = Vec::new();

    for n in 2usize.. {
        let factors: Vec<u64> = (0..n).map(|n| 10u64.pow(n as u32)).collect();

        // find the smallest possible number with the given number of digits.
        let smallest: u64 = iter::repeat(1u64)
            .zip(factors.iter().cloned())
            .map(|(n, order)| n * order)
            .sum();

        // compare with largest possible sum of the given power.
        let largest: u64 = iter::repeat(9u64).take(n).map(|v| v.pow(power)).sum();

        // if true there are no solutions with the given number of digits.
        if smallest > largest {
            break;
        }

        numperm(n, |values| {
            let digits = values
                .iter()
                .cloned()
                .zip(factors.iter().cloned())
                .map(|(v, f)| v * f)
                .sum();

            let powers = values.iter().cloned().map(|v| v.pow(power)).sum();

            if digits == powers {
                results.push(digits);
            }
        });
    }

    results.iter().cloned().sum::<u64>()
}

problem!{
    tests => [
        example1 => (run(4), 19316),
        q => {run(5), "46e68e4199ab0a663ab306651528b06756556c9f0d8b819095af45e036dfbe6b"},
    ];
}
