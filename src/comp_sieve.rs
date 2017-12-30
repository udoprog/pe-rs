//! A sieve to calculate low primes.

#![allow(unused)]

const START: u32 = 2;

use std::collections::HashMap;
use std::ops;
use std::hash;

pub trait Same<T> {}
impl<T> Same<T> for T {}

pub struct CompSieve<Idx, I> {
    composite: HashMap<Idx, Idx>,
    iter: I,
}

impl<Idx, I> CompSieve<Idx, I>
where
    Idx: PartialEq + Eq + hash::Hash,
{
    pub fn size(&self) -> usize {
        self.composite.len()
    }
}

impl<Idx> CompSieve<Idx, ops::RangeFrom<Idx>> {
    pub fn infinite<U: Same<Idx>>() -> CompSieve<Idx, ops::RangeFrom<Idx>>
    where
        Idx: From<u32> + Eq + hash::Hash,
    {
        CompSieve {
            iter: START.into()..,
            composite: HashMap::new(),
        }
    }

    pub fn bounded(upper: Idx) -> CompSieve<Idx, ops::Range<Idx>>
    where
        Idx: From<u32> + Eq + hash::Hash,
    {
        CompSieve {
            iter: START.into()..upper,
            composite: HashMap::new(),
        }
    }
}

impl<Idx, I> Iterator for CompSieve<Idx, I>
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
                return Some(n);
            } else {
                self.composite.insert(n * n, n);
            }
        }

        None
    }
}
