use std::mem;

struct Triangle {
    sum: u64,
    current: u64,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            sum: 0u64,
            current: 1u64,
        }
    }
}

impl Iterator for Triangle {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current + 1;
        self.sum += mem::replace(&mut self.current, next);
        Some(self.sum)
    }
}

fn run(limit: u64) -> u64 {
    for n in Triangle::new() {
        let mut divs = 0u64;

        let ceil = (n as f64).sqrt().ceil() as u64;

        for d in 1u64..ceil {
            if n % d == 0 {
                divs += 1;

                if d != (n / d) {
                    divs += 1;
                }
            }
        }

        if divs >= limit {
            return n;
        }
    }

    panic!("not found for limit: {}", limit);
}

problem!{
    tests => [
        example => (run(5), 28),
        q => {run(500), "3e7be445b6c19e6db58c2482005c1f78cb74011a4279249ca632011a9f1b61a2"},
    ];
}
