/// Keywords: digits

use p032::digits;

fn run() -> u64 {
    let mut count = 0;

    let mut numbers = 0u64..;
    let mut find = vec![1, 10, 100, 1000, 10_000, 100_000, 1_000_000].into_iter();

    let mut results = Vec::new();

    'outer: loop {
        let mut target = find.next().expect("first target");

        loop {
            let n = numbers.next().expect("missing number");

            let mut digs: Vec<u8> = digits(n).collect();
            digs.reverse();

            for d in digs {
                count += 1;

                if count == target {
                    results.push(d as u64);

                    if let Some(t) = find.next() {
                        target = t;
                    } else {
                        break 'outer;
                    }
                }
            }
        }
    }

    results.into_iter().fold(1u64, |a, b| a * b)
}

problem!{
    tests => [
        q => {run(), "d29d53701d3c859e29e1b90028eec1ca8e2f29439198b6e036c60951fb458aa1"},
    ];
}
