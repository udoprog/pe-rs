/// Keywords: permutations

use std::mem;

struct Permutations<T>
where
    T: Ord,
{
    input: Option<Vec<T>>,
}

impl<T> Permutations<T>
where
    T: Ord + Copy,
{
    pub fn new<I>(input: I) -> Permutations<T>
    where
        I: IntoIterator<Item = T>,
    {
        Permutations { input: Some(input.into_iter().collect()) }
    }

    /// The goal is to find the next index to swap out in order to do the least significant
    /// lexicographical 'bump. We call this the pivot.
    fn find_pivot(input: &[T]) -> Option<usize> {
        let mut it = input.iter().enumerate().rev();

        if let Some((_, mut p)) = it.next() {
            while let Some((i, n)) = it.next() {
                if n < p {
                    return Some(i);
                }

                p = n;
            }
        }

        None
    }

    /// Swap the pivot with the largest item beyond the pivot.
    ///
    /// In order to guarantee the next smallest lexicographical ordering, sort all elements beyond
    /// the pivot point.
    fn swap_pivot(input: &mut Vec<T>, pivot: usize) -> Option<Vec<T>> {
        let p = input[pivot];

        // only swap if a larger candidate can be found.
        if let Some(candidate) = input[pivot..]
            .iter()
            .cloned()
            .enumerate()
            .rev()
            .find(|v| v.1 > p)
            .map(|v| v.0)
        {
            let current = input.clone();
            input.swap(pivot, pivot + candidate);
            input[(pivot + 1)..].sort();
            return Some(current);
        }

        None
    }
}

impl<T> Iterator for Permutations<T>
where
    T: Copy + Ord,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if let Some(input) = self.input.as_mut() {
            if let Some(pivot) = Self::find_pivot(input) {
                if let Some(ret) = Self::swap_pivot(input, pivot) {
                    return Some(ret);
                }
            }
        }

        mem::replace(&mut self.input, None)
    }
}

fn run<I, T: Ord + Copy>(input: I, n: usize) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    Permutations::new(input).nth(n - 1).expect(
        "no such permutation",
    )
}

problem!{
    tests => [
        example1 => (
            Permutations::new(vec![0u8, 1, 2]).collect::<Vec<_>>(),
            vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0]
            ]
        ),
        q => {
            run(
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1_000_000
            ).iter().map(|n| n.to_string()).collect::<Vec<_>>().join(""),
            "4677b3d9daa3b30a9665e4558f826e04f7833dda886b8ef24f7176519a0db537"
        },
    ];
}
