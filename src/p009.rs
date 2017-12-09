fn run() -> u32 {
    for a in 1u32..=998 {
        for b in 1u32..=a {
            let c = 1000 - a - b;

            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c;
            }
        }
    }

    0
}

problem!{
    tests => [
        q => (31875000, run()),
    ];
}
