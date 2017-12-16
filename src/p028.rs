/// Keywords: spiral memory

fn run(layer: u32) -> u32 {
    // top-right corner is: n^2
    // simplified: n^2 + n^2 - (n-1) + n^2 - 2*(n-1) + n^2 - 3*(n-1) = 4*n^2 - 6n + 6
    (3u32..=layer).step_by(2).map(|n| 4 * n.pow(2) - 6 * n + 6).sum::<u32>() + 1u32
}

problem!{
    tests => [
        test1 => (run(1), 1),
        test2 => (run(3), 25),
        example1 => (run(5), 101),
        q => {run(1001), "261171a770d594f6a7fc76c1a839eda7f6dd4e9495e00e75048578fc86d8adf0"},
    ];
}
