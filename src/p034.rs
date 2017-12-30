/// Keywords: none

use digits::digits;

fn run() -> u64 {
    let mut factorials = [0u64; 10];
    factorials[0] = 1;

    for i in 1usize..10 {
        factorials[i] = factorials[i-1] * i as u64;
    }

    // From: 10^n < 9!*n
    // Upper limit: n - log(n) < log(9!) / log(10)
    // n ~ 7
    let upper = 10u64.pow(7);

    (3u64..upper)
        .filter(|d| *d == digits(*d).map(|d| factorials[d as usize]).sum())
        .sum()
}

problem!{
    tests => [
        q => {run(), "728b8d7d6d5d34cad9cbb7c3ea15f807ae57144594b1740b3c73b82314ccd1ed"},
    ];
}
