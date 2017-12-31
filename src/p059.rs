/// Keywords: none

use std::collections::HashMap;

fn run(input: &str) -> u64 {
    let data: Vec<u8> = input.trim().split(',').map(|v| v.parse::<u8>().expect("bad number")).collect();

    let mut key = Vec::new();

    for i in 0..3 {
        let mut freq: HashMap<u8, u32> = HashMap::new();

        for c in (i..data.len()).step_by(3) {
            *freq.entry(data[c]).or_insert_with(Default::default) += 1;
        }

        let mut sorted = freq.into_iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.1.cmp(&b.1));
        let space = sorted[sorted.len() - 1].0;

        // spaces probably has the highest frequency
        key.push(' ' as u8 ^ space);
    }

    let mut out = Vec::new();

    for (i, c) in data.iter().cloned().enumerate() {
        out.push((c ^ key[i % 3]) as char);
    }

    out.into_iter().map(|c| c as u64).sum::<u64>()
}

problem!{
    tests => [
        q => {run(include_str!("p059_cipher.txt")), "30f8673eb8490e9b2c07ee2f4de3fcad91b9fd8dd96511b60a9833d2fb884cd6"},
    ];
}
