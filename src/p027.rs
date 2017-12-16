/// Keywords: none

use sieve::Sieve;

fn run() -> Option<i64> {
    let mut max = 0;
    let mut biggest: Option<(i64, i64)> = None;

    // NB: b must be prime. consider n = 0.
    let b: Vec<i64> = Sieve::<i64, _>::bounded(1001).collect();

    for a in -999i64..=999 {
        for b in b.iter().cloned() {
            let n = (0i64..).map(|n| n.pow(2) + a * n + b).take_while(is_prime).count();

            if n > max {
                biggest = Some((a, b));
                max = n;
            }
        }
    }

    return biggest.map(|b| b.0 * b.1);

    fn is_prime(num: &i64) -> bool {
        let num = *num;

        if num <= 0 {
            return false;
        }

        let num = num as u64;
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
        q => {run().unwrap(), "e4110e0852a2f70703f0081fc91c4a20f595919a038729cb37c564d68b875c6f"},
    ];
}
