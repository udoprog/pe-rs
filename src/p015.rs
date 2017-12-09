use std::collections::{HashMap, VecDeque};

fn run(size: u32) -> u64 {
    let mut values: HashMap<(u32, u32), u64> = HashMap::new();
    values.insert((0, 0), 1);

    for idx in 1..=size {
        for x in (0..idx).rev() {
            let y = idx;
            let paths = calc_paths(&values, x, y);
            values.insert((x, y), paths);
        }

        for y in (0..idx).rev() {
            let x = idx;
            let paths = calc_paths(&values, x, y);
            values.insert((x, y), paths);
        }

        let paths = calc_paths(&values, idx, idx);
        values.insert((idx, idx), paths);
    }

    return *values.get(&(size, size)).expect("result");

    fn calc_paths(values: &HashMap<(u32, u32), u64>, x: u32, y: u32) -> u64 {
        let mut paths = 0u64;

        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        queue.push_back((x, y));

        while let Some((x, y)) = queue.pop_front() {
            if let Some(value) = values.get(&(x, y)) {
                paths += value;
                continue;
            }

            if x > 0 {
                queue.push_back((x - 1, y));
            }

            if y > 0 {
                queue.push_back((x, y - 1));
            }
        }

        paths
    }
}

problem!{
    tests => [
        example1 => (run(2), 6),
        example2 => (run(3), 20),
        example3 => (run(4), 70),
        example4 => (run(5), 252),
        example5 => (run(6), 924),
        q => {run(20), "7b8f812ca89e311e1b16b903de76fa7b0800a939b3028d9dc4d35f6fa4050281"},
    ];
}
