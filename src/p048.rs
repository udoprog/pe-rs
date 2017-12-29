/// Keywords: powers, modulo

fn run(limits: u64) -> u64 {
    let mut num = 0u64;

    for n in 1..=limits {
        let mut part = n;

        for _ in 1..n {
            part = part * n;
            part = part % 1_00000_00000;
        }

        num += part;
        num = num % 1_00000_00000;
    }

    num
}

problem!{
    tests => [
        test_1 => (run(100), 9027641920),
        q => {run(1000), "743d17cbff06ab458b99ecbb32e1d6bb9a7ff2ac804118f7743177dd969cfc61"},
    ];
}
