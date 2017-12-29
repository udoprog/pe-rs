/// Keywords: primes

use prime::is_prime;

fn run(count: u64) -> u64 {
    return (3u64..)
        .step_by(2)
        .filter(|v| is_prime(*v))
        .nth((count - 2) as usize)
        .unwrap();
}

problem!{
    tests => [
        example => (run(6), 13),
        q => {run(10001), "ecbe74e25cfa4763dbc304ccac2ffb9912e9625cd9993a84bd0dd6d7dc0ca021"},
    ];
}
