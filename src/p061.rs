/// Keywords: none

fn recurse(current: u64, path: Vec<(usize, u64)>, mut candidates: Vec<Option<&Vec<u64>>>) -> Vec<Vec<(usize, u64)>> {
    if candidates.iter().all(|c| c.is_none()) {
        let first = path[0].1;
        let last = path[path.len() - 1].1;

        if first / 100 != last % 100 {
            return vec![];
        }

        return vec![path];
    }

    let mut children = vec![];
    let mut index = 0;
    let end = current % 100;

    while index < candidates.len() {
        if let Some(values) = candidates[index].take() {
            let inner = candidates.clone();

            for v in values {
                if *v / 100 == end {
                    let mut path = path.clone();
                    path.push((index, *v));
                    children.extend(recurse(*v, path, inner.clone()));
                }
            }

            candidates[index] = Some(values);
        }

        index += 1;
    }

    children
}

fn run() -> u64 {
    let c3 = find_all(p3, 1000, 9999);
    let c4 = find_all(p4, 1000, 9999);
    let c5 = find_all(p5, 1000, 9999);
    let c6 = find_all(p6, 1000, 9999);
    let c7 = find_all(p7, 1000, 9999);
    let c8 = find_all(p8, 1000, 9999);

    let candidates = vec![Some(&c3), Some(&c4), Some(&c5), Some(&c6), Some(&c7)];

    let mut solutions = Vec::new();

    for start in c8 {
        solutions.extend(recurse(start, vec![(5, start)], candidates.clone()));
    }

    if solutions.len() > 1 {
        panic!("more than one solution");
    }

    return solutions[0].iter().map(|v| v.1).sum::<u64>();

    fn p3(n: u64) -> u64 {
        (n * (n+1)) / 2
    }

    fn p4(n: u64) -> u64 {
        n.pow(2)
    }

    fn p5(n: u64) -> u64 {
        (n * (3 * n - 1)) / 2
    }

    fn p6(n: u64) -> u64 {
        n * (2 * n - 1)
    }

    fn p7(n: u64) -> u64 {
        (n * (5 * n - 3)) / 2
    }

    fn p8(n: u64) -> u64 {
        n * (3 * n - 2)
    }
}

fn find_all<F>(f: F, lower: u64, upper: u64) -> Vec<u64> where F: Fn(u64) -> u64 {
    (1..).map(f).skip_while(|n| *n < lower).take_while(|n| *n <= upper).filter(|v| *v / 100 > 9 && *v % 100 > 9).collect()
}

problem!{
    tests => [
        q => {run(), "94e4fb283c1abcccae4b8b28e39a294a323cdc9732c3d3ce1133c518d0a286f6"},
    ];
}
