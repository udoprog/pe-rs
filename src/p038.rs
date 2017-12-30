/// Keywords: pandigital

use digits::digits;
use p032::is_pandigital;

fn run() -> u64 {
    let mut max = 0;

    for n in 1u64..100000 {
        let mut products = vec![];
        let mut digs = vec![];

        for d in 1.. {
            let mut ext: Vec<u8> = digits(n * d).collect();
            ext.reverse();

            if digs.len() + ext.len() > 9 {
                break;
            }

            products.push(d);
            digs.extend(ext);
        }

        if digs.len() != 9 || !is_pandigital(&digs) {
            continue;
        }

        let product = digs.iter().rev().cloned().zip(0u32..).map(|(a, b)| a as u64 * 10u64.pow(b)).sum();

        max = u64::max(max, product);
    }

    return max;
}

problem!{
    tests => [
        q => {run(), "b2004522103364a6e842b9d042c0707d79af68dec7810078729d061fb7948912"},
    ];
}
