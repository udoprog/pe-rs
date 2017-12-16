/// Keywords: combinations

use std::collections::HashSet;

fn dynamic(amount: u64) -> u64 {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut ways = vec![0u64; (amount + 1) as usize];

    ways[0] = 1;

    for coin in coins.iter().cloned() {
        for j in coin..=amount {
            ways[j as usize] += ways[(j - coin) as usize];
        }
    }

    ways[amount as usize]
}

fn brute() -> usize {
    let mut seen = HashSet::new();

    for p1 in 0..=200 {
        for p2 in 0..=100 {
            for p5 in 0..=40 {
                for p10 in 0..=20 {
                    for p20 in 0..=10 {
                        for p50 in 0..=4 {
                            for p100 in 0..=2 {
                                let sum = p1 + p2 * 2 + p5 * 5 + p10 * 10 + p20 * 20 + p50 * 50 + p100 * 100;

                                if sum == 200 {
                                    seen.insert((p1, p2, p5, p10, p20, p50, p100));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    seen.len() + 1
}

problem!{
    tests => [
        q_dynamic => {dynamic(200), "8de34b4ba97b184c7a2096b9266776175242b87d67bc8d77d7289be6f70cd105"},
        q => {brute(), "8de34b4ba97b184c7a2096b9266776175242b87d67bc8d77d7289be6f70cd105"},
    ];
}
