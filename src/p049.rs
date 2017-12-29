/// Keywords: none

use p032::digits;
use prime::is_prime;

fn run() -> u64 {
    let f = 3330;

    for a in 1000u64..(10000 - f * 2) {
        let b = a + (f * 1);
        let c = a + (f * 2);

        let mut a_d: Vec<u8> = digits(a).collect();
        a_d.sort();

        let mut b_d: Vec<u8> = digits(b).collect();
        b_d.sort();

        let mut c_d: Vec<u8> = digits(c).collect();
        c_d.sort();

        if !(a_d == b_d && a_d == c_d) {
            continue;
        }

        // skip example
        if a == 1487 {
            continue;
        }

        if is_prime(a) && is_prime(b) && is_prime(c) {
            let mut sum = 0u64;
            sum += a * 1_0000_0000;
            sum += b * 1_0000;
            sum += c;
            return sum;
        }
    }

    0
}

problem!{
    tests => [
        q => {run(), "47c6094ff1ff6e37788def89190c8256619ef1511681c503fea02c171569d16e"},
    ];
}
