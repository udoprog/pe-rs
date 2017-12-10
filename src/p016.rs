use std::iter::repeat;

pub fn run<I>(factors: I) -> u64 where I: IntoIterator<Item = u64> {
    let mut digits = vec![1u8];

    for fac in factors.into_iter() {
        let mut carry = 0u64;

        for d in digits.iter_mut() {
            let v = (*d as u64).checked_mul(fac)
                .and_then(|v| v.checked_add(carry))
                .expect("overflow");

            *d = (v % 10) as u8;
            carry = v / 10;
        }

        while carry > 0 {
            digits.push((carry % 10) as u8);
            carry = carry / 10;
        }
    }

    digits.into_iter().map(|v| v as u64).sum()
}

problem!{
    tests => [
        example1 => (run(repeat(2).take(15)), 26),
        example2 => (run(repeat(9).take(1000)), 4338),
        example3 => (run(repeat(255).take(1000)), 10872),
        q => {run(repeat(2).take(1000)), "a6f988d30328bd706c66f8ac0d92aac21dd732149cdd69cb31f459dca20c5abe"},
    ];
}
