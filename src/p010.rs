use sieve::FixedSieve;

fn run(limit: u64) -> u64 {
    FixedSieve::new(limit).sum()
}

problem!{
    tests => [
        example => (17, run(10)),
        q => {run(2_000_000), "bed2d160e02f0540f19a64ca738aacb79cfcd08ba7e2421567b16cb6e7e3e90e"},
    ];
}
