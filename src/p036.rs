/// Keywords: palindrome, inverse, p004

fn run(limit: u64) -> u64 {
    let mut sum = 0u64;

    for i in 0u64..limit {
        if inverse(i, 10) == i && inverse(i, 2) == i {
            sum += i;
        }
    }

    return sum;

    fn inverse(mut n: u64, base: u64) -> u64 {
        let mut inverse = 0;

        while n > 0 {
            inverse *= base;
            inverse += n % base;
            n /= base;
        }

        inverse
    }
}

problem!{
    tests => [
        example => (run(1_000), 1772),
        q => {run(1_000_000), "9480c0160719234b57defc0681c0949a175ffb3ff4a3bf5e8163ac843f383f35"},
    ];
}
