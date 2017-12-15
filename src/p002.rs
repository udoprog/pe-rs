/// Keywords: sequences, fib

use std::ops;
use std::mem;

pub struct Fib<N> {
    v: (N, N),
}

impl<N> Fib<N>
where
    N: From<u32>,
{
    pub fn new() -> Fib<N> {
        Fib { v: (1.into(), 1.into()) }
    }
}

impl<N> Iterator for Fib<N>
where
    N: Clone + ops::Add<Output = N>,
{
    type Item = N;

    fn next(&mut self) -> Option<N> {
        let next = {
            let (ref a, ref b) = self.v;
            (b.clone(), a.clone() + b.clone())
        };

        Some(mem::replace(&mut self.v, next).0)
    }
}

fn run(limit: u32) -> u32 {
    Fib::<u32>::new()
        .take_while(|v| *v < limit)
        .filter(|v| v % 2 == 0)
        .sum()
}

problem!{
    tests => [
        example => (run(1000), 798),
        q => {run(4000000), "1f5882e19314ac13acca52ad5503184b3cb1fd8dbeea82e0979d799af2361704"},
    ];
}
