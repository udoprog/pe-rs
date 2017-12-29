/// Keywords: none

use sieve::Sieve;
use p032::digits;
use combination::Combinations;
use std::collections::HashMap;

use self::Digit::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Digit {
    Number(u8),
    Any,
}

fn run(target: usize) -> u64 {
    let mut found: HashMap<Vec<Digit>, usize> = HashMap::new();
    let mut first: HashMap<Vec<Digit>, u64> = HashMap::new();

    'outer: for p in Sieve::infinite::<u64>() {
        let digs: Vec<u8> = digits(p).collect();
        let pos: Vec<usize> = (0..digs.len()).collect();

        for l in 1..pos.len() {
            for pos in Combinations::new(&pos, l) {
                let comp = digs[pos[0]];

                if !pos[1..].iter().all(|d| digs[*d] == comp) {
                    continue;
                }

                let mut digs: Vec<Digit> = digs.iter().cloned().map(Number).collect();

                for p in pos.iter().cloned() {
                    digs[p] = Any;
                }

                let first = first.entry(digs.clone()).or_insert(p);
                let n = found.entry(digs.clone()).or_insert_with(Default::default);
                *n += 1;

                if *n == target {
                    return *first;
                }
            }
        }
    }

    panic!("no solution :(");
}

problem!{
    tests => [
        example_1 => (run(3), 11),
        example_2 => (run(7), 56003),
        q => {run(8), "d17cec28356b4f9a7f1ec0f20cca4c89e270aeb0e75d70d485b05bb1f28e9f6d"},
    ];
}
