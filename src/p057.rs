/// Keywords: none

use num::BigUint;
use std::mem;

fn run() -> u64 {
    let mut count = 0;

    let mut num: BigUint = 5u32.into();
    let mut den: BigUint = 2u32.into();

    for _ in 0..999 {
        mem::swap(&mut num, &mut den);
        num += den.clone() * 2u32;

        // final step
        {
            let mut num = num.clone();
            let mut den = den.clone();
            mem::swap(&mut num, &mut den);
            num += den.clone();

            if num.to_radix_le(10).len() > den.to_radix_le(10).len() {
                count += 1;
            }
        }
    }

    count
}

problem!{
    tests => [
        q => {run(), "620c9c332101a5bae955c66ae72268fbcd3972766179522c8deede6a249addb7"},
    ];
}
