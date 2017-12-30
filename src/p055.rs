/// Keywords: none

use num::BigUint;
use digits::digits;

fn run() -> u64 {
    let mut count = 0;

    'skip: for n in 1u64..10000 {
        let mut n: BigUint = n.into();
        n += add(n.clone());

        for _ in 0..50 {
            if n == inverse(n.clone()) {
                continue 'skip;
            }

            n += add(n.clone());
        }

        count += 1;
    }

    return count;

    fn add(n: BigUint) -> BigUint {
        let add: BigUint = 0u32.into();
        let add = digits(n).fold(add, |a, b| (a + b) * 10u32);
        add / 10u32
    }

    fn inverse(mut n: BigUint) -> BigUint {
        let mut inverse: BigUint = 0u32.into();
        let zero: BigUint = 0u32.into();
        let ten: BigUint = 10u32.into();

        while n > zero {
            inverse *= &ten;
            inverse += n.clone() % &ten;
            n /= &ten;
        }

        inverse
    }
}

problem!{
    tests => [
        q => {run(), "9f484139a27415ae2e8612bf6c65a8101a18eb5e9b7809e74ca63a45a65f17f4"},
    ];
}
