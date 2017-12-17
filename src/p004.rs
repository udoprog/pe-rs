/// Keywords: palindrome

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
        example => (run(2), 9009),
        q => {run(3), "aa74f52b4c428d89606b411bc165eb81a6266821ecc9b4f30cdb70c5c930f4d9"},
    ];
}
