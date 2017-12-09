use std::mem;

pub struct Fib {
    v: (u32, u32),
}

impl Fib {
    pub fn new() -> Fib {
        Fib { v: (1, 1) }
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let next = (self.v.1, self.v.0 + self.v.1);
        Some(mem::replace(&mut self.v, next).1)
    }
}

fn run(limit: u32) -> u32 {
    Fib::new()
        .take_while(|v| *v < limit)
        .filter(|v| v % 2 == 0)
        .sum()
}

problem!{
    tests => [
        example => (run(1000), 798),
        q => (run(4000000), 4613732),
    ];
}
