/// Keywords: sequences, fib

use p002::Fib;
use num_bigint::BigUint;

fn run(digits: u64) -> u64 {
    let target = (0..(digits - 1)).fold(1u32.into(), |s, _| s * 10u32);

    Fib::<BigUint>::new()
        .enumerate()
        .find(|v| v.1 > target)
        .map(|v| v.0 + 1)
        .expect("no result") as u64
}

problem!{
    tests => [
        example1 => (
            run(3),
            12
        ),
        q => {run(1000), "7d398da8791745001b3d1c41030676d1c036687eb1ab32e0b5a1832e7579c073"},
    ];
}
