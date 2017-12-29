pub fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }

    if num == 2 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    let upper = (num as f64).sqrt().ceil() as u64 + 1;
    !(3..upper).step_by(2).any(|i| num % i == 0)
}

#[cfg(test)]
mod tests {
    use super::is_prime;
    use sieve::Sieve;
    use std::collections::HashSet;

    #[test]
    fn test_primes() {
        let mut numbers: HashSet<u64> = (0u64..100000).collect();

        for p in Sieve::bounded(100000u64) {
            assert_eq!(is_prime(p), true, "should be prime: {}", p);
            numbers.remove(&p);
        }

        for c in numbers.into_iter() {
            assert_eq!(is_prime(c), false, "should be composite: {}", c);
        }
    }
}
