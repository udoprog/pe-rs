use p012::divisors;

pub fn proper_divisors(n: u64) -> impl Iterator<Item = u64> {
    divisors(n).filter(move |d| *d < n)
}

fn is_amicable(n: u64) -> bool {
    let a = proper_divisors(n).sum();
    let b = proper_divisors(a).sum();
    n == b && a != b
}

fn run() -> u64 {
    (1u64..10000).filter(|d| is_amicable(*d)).sum()
}

problem!{
    tests => [
        proper_divisors_1 => (proper_divisors(6).collect::<Vec<_>>(), vec![1, 2, 3]),
        proper_divisors_2 => (proper_divisors(196).collect::<Vec<_>>(), vec![1, 2, 98, 4, 49, 7, 28, 14]),
        example1 => (is_amicable(284), true),
        q => {run(), "e8c6ef4a1736a245b5682e0262c5c43862cfb233ca5e286be2f5bb4d8a974ecf"},
    ];
}
