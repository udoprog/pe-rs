/// Keywords: none

use num::BigUint;
use num::pow::pow;

use p055::big_digits;

fn run() -> u64 {
    let mut max = 0u64;

    for a in 0u32..100 {
        for b in 0usize..100 {
            let a: BigUint = a.into();
            max = u64::max(max, big_digits(pow(a, b)).map(|b| b as u64).sum::<u64>());
        }
    }

    max
}

problem!{
    tests => [
        q => {run(), "3658d7fa3c43456f3c9c87db0490e872039516e6375336254560167cc3db2ea2"},
    ];
}
