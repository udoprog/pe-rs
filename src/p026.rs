/// Keywords: divisors, fractions

use std::mem;

fn cycle(div: u64) -> usize {
    let mut digits = vec![0usize; div as usize];
    let mut num = 1;

    for cycle in 0usize.. {
        num = (num * 10) % div;

        if num == 0 {
            break;
        }

        let digit = mem::replace(&mut digits[num as usize], cycle);

        if digit > 0 {
            return cycle - digit;
        }
    }

    0
}

fn run(limit: u64) -> Option<u64> {
    (2u64..limit).max_by_key(|v| cycle(*v))
}

problem!{
    tests => [
        example1 => (run(11), Some(7)),
        cycle1 => (cycle(876), 8),
        q => {run(1000).unwrap(), "fbe10beedf9d29cf53137ba38859ffd1dbe7642cedb7ef0a102a3ab109b47842"},
    ];
}
