#![allow(unused)]

use std::ops;
use core::nonzero;

#[derive(Debug, PartialEq, Eq)]
pub enum Carry {
    Overflow,
    Done,
}

pub trait Oper<T> {
    fn increment(&self, T) -> (T, Carry);
}

pub fn permutate<T, O: Oper<T>>(values: &mut [T], oper: &O) -> bool
where
    T: Clone,
{
    let mut nth = 0usize;
    let mut carry = Carry::Overflow;

    while carry == Carry::Overflow && nth < values.len() {
        let (v, c) = oper.increment(values[nth].clone());
        values[nth] = v;
        carry = c;
        nth += 1;
    }

    carry != Carry::Overflow
}

/// A permutation oper that wraps around a number.
pub struct Wrapping<T>(pub T, pub T);

impl<T> Oper<T> for Wrapping<T>
where
    T: Copy
        + nonzero::Zeroable
        + ops::Add<Output = T>
        + ops::Rem<Output = T>
        + ops::Div<Output = T>,
{
    fn increment(&self, value: T) -> (T, Carry) {
        let next = value + self.1;

        let v = next % self.0;
        let carry = next / self.0;

        if !carry.is_zero() {
            (v, Carry::Overflow)
        } else {
            (v, Carry::Done)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        let oper = Wrapping(2u32, 1);
        let mut values = vec![0, 0, 0];
        assert_eq!(permutate(&mut values, &oper), true);
        assert_eq!(values, vec![1, 0, 0]);
        assert_eq!(permutate(&mut values, &oper), true);
        assert_eq!(values, vec![0, 1, 0]);
        assert_eq!(permutate(&mut values, &oper), true);
        assert_eq!(values, vec![1, 1, 0]);
        assert_eq!(permutate(&mut values, &oper), true);
        assert_eq!(values, vec![0, 0, 1]);
        assert_eq!(permutate(&mut values, &oper), true);
        assert_eq!(values, vec![1, 0, 1]);
        assert_eq!(permutate(&mut values, &oper), true);
        assert_eq!(values, vec![0, 1, 1]);
        assert_eq!(permutate(&mut values, &oper), true);
        assert_eq!(values, vec![1, 1, 1]);
        assert_eq!(permutate(&mut values, &oper), false);
        assert_eq!(values, vec![0, 0, 0]);
    }
}
