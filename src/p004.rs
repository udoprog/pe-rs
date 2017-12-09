fn run(digits: u32) -> u64 {
    let upper = 10u64.pow(digits);
    let lower = 10u64.pow(digits - 1);

    let mut largest = 0;

    for a in (lower..upper).rev() {
        for b in (a..upper).rev() {
            let v = a * b;

            if v == inverse(v) {
                largest = u64::max(largest, v);
            }
        }
    }

    return largest;

    fn inverse(mut n: u64) -> u64 {
        let mut inverse = 0;

        while n > 0 {
            inverse *= 10;
            inverse += n % 10;
            n /= 10;
        }

        inverse
    }
}

problem!{
    tests => [
        example => (9009, run(2)),
        q => (906609, run(3)),
    ];
}
