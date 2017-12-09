fn run(end: u32) -> u32 {
    (1u32..end).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

problem!{
    tests => [
        example => (run(10), 23),
        q => {run(1000), "c0b20f4665d0388d564f0b6ecf3edc9f9480cb15fff87198b95701d9f5fe1f7b"},
    ];
}
