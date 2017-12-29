/// Keywords: pentagonal

fn run() -> u64 {
    for i in 2.. {
        for j in 1..i {
            let a = triangle(i);
            let b = triangle(j);

            let sum = a + b;
            let diff = if a > b { a - b } else { b - a };

            if solve(sum).is_some() && solve(diff).is_some() {
                return diff;
            }
        }
    }

    panic!("no solution :(");

    fn triangle(n: u64) -> u64 {
        (3 * n.pow(2) - n) / 2
    }

    fn solve(v: u64) -> Option<u64> {
        let sol = (0.5f64 + (0.25f64 + 6f64 * (v as f64)).sqrt()) / 3f64;

        if sol == sol.round() {
            Some(sol as u64)
        } else {
            None
        }
    }
}

problem!{
    tests => [
        q => {run(), "97e2524fd3796e83b06c0f89fdcb16e4c544e76e9c0496f57ac84834869f4cc3"},
    ];
}
