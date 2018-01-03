/// Keywords: none

use std::mem;
use num::BigUint;
use digits::digits;

fn run(limit: u64) -> u64 {
    let mut nums = (1u64..limit).rev().map(|i| {
        let q = if (i + 1) % 3 == 0 {
            2 * (i + 2) / 3
        } else {
            1
        };

        q.into()
    });

    let mut num: BigUint = nums.next().expect("first number");
    let mut den: BigUint = 1u32.into();

    for q in nums {
        num += q * den.clone();
        mem::swap(&mut num, &mut den);
    }

    num += den.clone() * 2u64;
    digits(num).map(|d| d as u64).sum()
}

problem!{
    tests => [
        example => (run(10), 17),
        q => {run(100), "1c6c0bb2c7ecdc3be8e134f79b9de45155258c1f554ae7542dce48f5cc8d63f0"},
    ];
}
