fn run(mut number: u64) -> u64 {
    let mut factor = 1u64;

    let ceil = f64::ceil(f64::sqrt(number as f64)) as u64;

    for n in 2u64..ceil {
        while number % n == 0 {
            factor = n;
            number = number / n;
        }
    }

    factor
}

problem!{
    tests => [
        example => (run(13195), 29),
        q => {run(600851475143), "5c09f0554518a413e58e6bc5964ba90655713483d0b2bbc94572ad6b0b4dda28"},
    ];
}
