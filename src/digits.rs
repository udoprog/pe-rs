use num::{ToPrimitive, Zero};
use std::cmp;
use std::ops;

pub struct Digits<T> {
    value: T,
}

impl<T> Iterator for Digits<T>
where
    T: Clone,
    T: cmp::Ord,
    T: Zero,
    T: ops::Rem<T, Output = T>,
    T: ops::Div<T, Output = T>,
    T: ToPrimitive,
    u32: Into<T>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value > T::zero() {
            let o = (self.value.clone() % 10u32.into()).to_u8().expect("bad u8");
            self.value = self.value.clone() / 10u32.into();
            Some(o)
        } else {
            None
        }
    }
}

pub fn digits<T>(value: T) -> Digits<T> {
    Digits { value: value }
}

#[cfg(test)]
mod tests {
    use super::digits;

    #[test]
    fn test_u64() {
        assert_eq!(vec![4, 3, 2, 1], digits(1234u32).collect::<Vec<_>>());
    }

    #[test]
    fn test_biguint() {
        use num::BigUint;

        let b: BigUint = 1234u32.into();
        let digs = digits(b).collect::<Vec<u8>>();
        assert_eq!(vec![4, 3, 2, 1], digs);
    }
}
