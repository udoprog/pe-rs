/// Keywords: none

use num::BigUint;
use num::pow::pow;

fn run() -> u64 {
    let mut count = 0;

    for power in 1usize..30 {
        'outer: for n in 1u64.. {
            let value: BigUint = pow(n.into(), power);
            let len = value.to_string().len();

            if len == power {
                count += 1;
            }

            if len > power {
                break;
            }
        }
    }

    count
}

problem!{
    tests => [
        q => {run(), "0e17daca5f3e175f448bacace3bc0da47d0655a74c8dd0dc497a3afbdad95f1f"},
    ];
}
