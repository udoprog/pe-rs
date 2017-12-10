use std::mem;

/// Divisors for the given number.
pub fn divisors(n: u64) -> impl Iterator<Item = u64> {
    let ceil = (n as f64).sqrt() as u64 + 1;

    (1u64..ceil).filter(move |d| n % d == 0).flat_map(move |d| {
        let mut more = vec![d];

        if d != (n / d) {
            more.push(n / d);
        }

        more.into_iter()
    })
}

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
        if divisors(n).count() as u64 > limit {
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
