fn run(end: u32) -> u32 {
    (1u32..end).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

problem!{
    tests => [
        example => (run(10), 23),
        q => (run(1000), 233168),
    ];
}
