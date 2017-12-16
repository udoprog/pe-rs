/// Keywords: primes

fn run(count: u64) -> u64 {
    return (3u64..)
        .step_by(2)
        .filter(is_prime)
        .nth((count - 2) as usize)
        .unwrap();

    fn is_prime(num: &u64) -> bool {
        let num = *num;
        let upper = (num as f64).sqrt().ceil() as u64;

        for i in 3..=upper {
            if num % i == 0 {
                return false;
            }
        }

        true
    }
}

problem!{
    tests => [
        example => (run(6), 13),
        q => {run(10001), "ecbe74e25cfa4763dbc304ccac2ffb9912e9625cd9993a84bd0dd6d7dc0ca021"},
    ];
}
