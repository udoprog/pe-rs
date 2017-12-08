fn run(end: u32) -> u32 {
    (1u32..end).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

problem!{
    tests => [
        example => (23, run(10)),
        q => (233168, run(1000)),
    ];
}
