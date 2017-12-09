use std::io::{Cursor, BufRead, BufReader};

static INPUT: &str = include_str!("p013_input.txt");

/// NB: any digits beyond the `w + 1` first have no impact on the final result.
///
/// Consider:
///
/// 99900 + 99900 = 199|800
/// 99999 + 99999 = 199|998
/// 999   + 999   = 199|8
fn run(w: usize) -> u64 {
    assert!(w > 0, "bad input: {}", w);

    let mut sum = 0u64;
    let m = 10u64.pow(w as u32);

    for line in BufReader::new(Cursor::new(INPUT)).lines() {
        let line = line.expect("bad line");
        let line = line.trim();
        let num = line[..=w].parse::<u64>().expect("bad number");
        sum += num;
    }

    while sum > m {
        sum /= 10u64;
    }

    sum
}

problem!{
    tests => [
        q => {run(10), "3cb265a96c5645a9ad11d47551f015c25f3f99792c951617656d84626fbc4868"},
        example1 => {run(1), "ef2d127de37b942baad06145e54b0c619a1f22327b2ebbcfbec78f5564afe39d"},
        example2 => {run(2), "02d20bbd7e394ad5999a4cebabac9619732c343a4cac99470c03e23ba2bdc2bc"},
        example3 => {run(3), "d40fbd13d527595c47eacbf0d7c87d256139d9d45261c25c2840d30a4756495b"},
    ];
}
