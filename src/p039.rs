/// Keywords: triangle

fn brute() -> u64 {
    let mut max = (0, 0);

    for p in 3u64..=1000 {
        let mut count = 0;

        for a in 1..p {
            for b in 1..(p - a) {
                let c = p - a - b;

                if a.pow(2) + b.pow(2) == c.pow(2) {
                    count += 1;
                }
            }
        }

        if count > max.0 {
            max = (count, p);
        }
    }

    max.1
}

problem!{
    tests => [
        q => {brute(), "fd0f7e53c5b02b688a57ee37f3d52065cb168a7b9fd5a3abd93d37e1559fbd30"},
    ];
}
