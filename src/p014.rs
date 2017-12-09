use std::mem;
use std::collections::HashMap;

struct Seq {
    n: u64,
}

impl Seq {
    pub fn new(n: u64) -> Seq {
        Seq {
            n: n
        }
    }
}

impl Iterator for Seq {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;

        if n == 0 {
            return None;
        }

        if n == 1 {
            return Some(mem::replace(&mut self.n, 0));
        }

        let next = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };

        Some(mem::replace(&mut self.n, next))
    }
}

fn run(limit: u64) -> u64 {
    let mut known: HashMap<u64, usize> = HashMap::new();
    let mut length = 0usize;
    let mut longest = (0usize, 1u64);

    for n in 2..=limit {
        let mut out = Vec::new();
        let mut found = 0usize;

        for (i, s) in Seq::new(n).enumerate() {
            if let Some(k) = known.get(&s) {
                found = *k;
                break;
            }

            length = i;
            out.push(s);
        }

        for (offset, value) in out.into_iter().rev().enumerate() {
            known.insert(value, found + offset);
        }

        let current = length + found;

        if longest.0 < current {
            longest = (current, n);
        }
    }

    longest.1
}

problem!{
    tests => [
        example => (run(1), 1),
        example2 => (run(2), 2),
        example3 => (run(3), 3),
        seq => (Seq::new(13).collect::<Vec<_>>(), vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]),
        q => {run(999_999), "78a262dd40eba0f7195686ec7f3891a39437523456f8d16fa9065a34409eeac6"},
    ];
}
