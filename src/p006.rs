/// Keywords: squares

fn run(count: u64) -> u64 {
    (1u64..=count).sum::<u64>().pow(2) - (1u64..=count).map(|v| v.pow(2)).sum::<u64>()
}

problem!{
    tests => [
        example => (run(10), 2640),
        q => {run(100), "537942be3eb323c507623a6a73fa87bf5aeb97b7c7422993a82aa7c15f6d9cd6"},
    ];
}
