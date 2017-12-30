use num::ToPrimitive;
use std::cmp;
use std::ops;

pub struct Digits<T> {
    value: T,
    zero: T,
}

impl<T> Iterator for Digits<T>
    where T: Clone,
          T: cmp::Ord,
          T: From<u32>,
          T: ops::Rem<u32, Output=T>,
          T: ops::Div<u32, Output=T>,
          T: ToPrimitive
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value > self.zero {
            let o = (self.value.clone() % 10u32).to_u8().expect("bad u8");
            self.value = self.value.clone() / 10u32;
            Some(o)
        } else {
            None
        }
    }
}

pub fn digits<T>(value: T) -> Digits<T>
    where T: Clone,
          T: cmp::Ord,
          T: From<u32>,
          T: ops::Rem<u32, Output=T>,
          T: ops::Div<u32, Output=T>,
          T: ToPrimitive
{
    Digits {
        value: value,
        zero: 0u32.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::digits;

    #[test]
    fn test_u64() {
        assert_eq!(vec![4, 3, 2, 1], digits(1234).collect::<Vec<_>>());
    }

    #[test]
    fn test_biguint() {
        use num::BigUint;

        let b: BigUint = 1234u32.into();
        assert_eq!(vec![4, 3, 2, 1], digits(b).collect::<Vec<_>>());
    }
}
