//! A sieve to calculate low primes.

#![allow(unused)]

const START: u32 = 2;

use std::collections::HashMap;
use std::ops;
use std::hash;

pub trait Same<T> {}
impl<T> Same<T> for T {}

pub struct Sieve<Idx, I> {
    composite: HashMap<Idx, Idx>,
    iter: I,
}

impl<Idx, I> Sieve<Idx, I>
where
    Idx: PartialEq + Eq + hash::Hash,
{
    pub fn size(&self) -> usize {
        self.composite.len()
    }
}

impl<Idx> Sieve<Idx, ops::RangeFrom<Idx>> {
    pub fn infinite<U: Same<Idx>>() -> Sieve<Idx, ops::RangeFrom<Idx>>
    where
        Idx: From<u32> + Eq + hash::Hash,
    {
        Sieve {
            iter: START.into()..,
            composite: HashMap::new(),
        }
    }

    pub fn bounded(upper: Idx) -> Sieve<Idx, ops::Range<Idx>>
    where
        Idx: From<u32> + Eq + hash::Hash,
    {
        Sieve {
            iter: START.into()..upper,
            composite: HashMap::new(),
        }
    }
}

impl<Idx, I> Iterator for Sieve<Idx, I>
where
    I: Iterator<Item = Idx>,
    Idx: Eq + hash::Hash + Copy + ops::Add<Output = Idx> + ops::Mul<Output = Idx>,
{
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.iter.next() {
            if let Some(value) = self.composite.remove(&n) {
                let mut key = n + value;

                while self.composite.contains_key(&key) {
                    key = key + value;
                }

                self.composite.insert(key, value);
            } else {
                self.composite.insert(n * n, n);
                return Some(n);
            }
        }

        None
    }
}

pub struct FixedSieve {
    sieve: Vec<bool>,
    upper: u64,
    n: u64,
}

impl FixedSieve {
    pub fn new(upper: u64) -> FixedSieve {
        FixedSieve {
            sieve: vec![false; (upper - 1) as usize],
            upper: upper,
            n: START as u64,
        }
    }
}

impl Iterator for FixedSieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = self.n;

        while n <= self.upper {
            if !self.sieve[(n - START as u64) as usize] {
                let mut v = n;

                while v <= self.upper {
                    self.sieve[(v - START as u64) as usize] = true;
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
