/// Keywords: none

use sieve::Sieve;

fn numgen() -> impl Iterator<Item = (u64, u32)> {
    let mut s = Sieve::infinite::<u64>();
    let mut primes: Vec<u64> = Vec::new();

    primes.push(s.next().expect("one prime"));

    (1u64..).map(move |num| {
        let mut prime_factors = 0;

        let mut n = num;

        while *primes.last().expect("no number") < (num / 2) + 1 {
            primes.push(s.next().expect("no prime"));
        }

        for p in primes.iter().cloned() {
            if n % p == 0 {
                n = n / p;

                prime_factors += 1;

                while n % p == 0 {
                    n = n / p;
                }
            }
        }

        (num, prime_factors)
    })
}

fn run() -> u64 {
    let mut con = 0;

    for (d, f) in numgen() {
        if f == 4 {
            con += 1;
        } else {
            con = 0;
        }

        if con == 4 {
            return d - 3;
        }
    }

    panic!("nothing found :(");
}

problem!{
    tests => [
        q => {run(), "c7274da71333bd93201fa1e05b1ed54e0074d83f259bd7148c70ddc43082bde1"},
    ];
}
