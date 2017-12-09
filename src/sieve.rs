//! A sieve to calculate low primes.

const START: u64 = 2;

pub struct Sieve {
    sieve: Vec<bool>,
    upper: u64,
    n: u64,
}

impl Sieve {
    pub fn new(upper: u64) -> Sieve {
        Sieve {
            sieve: vec![false; (upper - 1) as usize],
            upper: upper,
            n: START,
        }
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = self.n;

        while n <= self.upper {
            if !self.sieve[(n - START) as usize] {
                let mut v = n;

                while v <= self.upper {
                    self.sieve[(v - START) as usize] = true;
                    v += n;
                }

                self.n = n + 1;
                return Some(n);
            }

            n += 1;
        }

        self.n = n;
        None
    }
}

