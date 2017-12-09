fn run(count: u64) -> u64 {
    (1u64..=count).sum::<u64>().pow(2) - (1u64..=count).map(|v| v.pow(2)).sum::<u64>()
}

problem!{
    tests => [
        example => (2640, run(10)),
        q => (25164150, run(100)),
    ];
}
