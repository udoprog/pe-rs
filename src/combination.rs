// ported from: https://docs.python.org/2/library/itertools.html#itertools.combinations
pub struct Combinations<'a, T: 'a> {
    pool: &'a [T],
    indices: Vec<usize>,
    len: usize,
    first: bool,
}

impl<'a, T> Combinations<'a, T> {
    pub fn new(pool: &'a [T], len: usize) -> Combinations<'a, T> {
        Combinations {
            pool: pool,
            indices: (0..len).collect(),
            len: len,
            first: true,
        }
    }
}

impl<'a, T> Iterator for Combinations<'a, T>
where
    T: Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        } else {
            if let Some(i) = find_i(&self.pool, &self.indices, self.len) {
                self.indices[i] += 1;

                for j in (i + 1)..self.len {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
            } else {
                return None;
            }
        }

        return Some(self.indices.iter().cloned().map(|i| self.pool[i]).collect());

        fn find_i<T>(pool: &[T], indices: &[usize], len: usize) -> Option<usize> {
            for i in (0..len).rev() {
                if indices[i] != i + pool.len() - len {
                    return Some(i);
                }
            }

            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Combinations;

    #[test]
    fn test_combinations() {
        let pool = vec![1, 2, 3, 4];

        for c in Combinations::new(&pool, 2) {
            println!("comb = {:?}", c);
        }
    }
}
