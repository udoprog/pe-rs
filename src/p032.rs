/// Keywords: combinations, digits, pandigital

use std::collections::HashSet;
use digits::digits;

use std::mem;

/// Only lower divisors for the given number.
pub fn lower_divisors(n: u64) -> impl Iterator<Item = u64> {
    let ceil = (n as f64).sqrt() as u64 + 1;
    (1u64..ceil).filter(move |d| n % d == 0)
}

pub fn is_pandigital(digits: &[u8]) -> bool {
    let mut found = [false; 9];

    for d in digits {
        if *d == 0 {
            return false;
        }

        if mem::replace(&mut found[*d as usize - 1], true) {
            return false;
        }
    }

    found.into_iter().take(digits.len()).all(|n| *n)
}

fn run() -> u64 {
    let mut result = HashSet::new();

    for n in 1..10000 {
        let digs: Vec<u8> = digits(n).collect();

        for div in lower_divisors(n) {
            let mut digs = digs.clone();
            digs.extend(digits(div));
            digs.extend(digits(n / div));

            if digs.len() != 9 || !is_pandigital(&digs) {
                continue;
            }

            result.insert(n);
        }
    }

    result.into_iter().sum::<u64>()
}

problem!{
    tests => [
        digits_1 => (digits(123456789u32).collect::<Vec<_>>(), vec![9,8,7,6,5,4,3,2,1]),
        is_pandigital_1 => (is_pandigital(&[1, 2, 3, 5]), false),
        is_pandigital_2 => (is_pandigital(&[1, 2, 3, 4, 5]), true),
        is_pandigital_3 => (is_pandigital(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), true),
        pandigital_false => (is_pandigital(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), false),
        q => {run(), "0d246750daa7f1b367a21f55da454ddc8f62e0a95d163062e9b9273320d5130f"},
    ];
}
