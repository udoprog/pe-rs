use sieve::Sieve;

fn run(limit: u64) -> u64 {
    Sieve::new(limit).sum()
}

problem!{
    tests => [
        example => (17, run(10)),
        q => (run(2_000_000), 142913828922),
    ];
}
