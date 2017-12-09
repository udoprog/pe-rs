use std::io::{BufRead, BufReader, Cursor};
use std::cell::RefCell;
use std::rc::Rc;

type Line = Rc<RefCell<Vec<u64>>>;

static TRIANGLE: &str = include_str!("p067_triangle.txt");

fn run() -> u64 {
    let reader = BufReader::new(Cursor::new(TRIANGLE));

    let mut lines: Vec<(Option<Line>, Line)> = Vec::new();

    // reference to previous line.
    let mut prev: Option<Line> = None;

    for line in reader.lines() {
        let line = line.expect("line");

        let line = line.trim()
            .split_whitespace()
            .map(|d| d.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .expect("row of digits");

        let current = Rc::new(RefCell::new(line));
        lines.push((prev.clone(), current.clone()));
        prev = Some(current.clone());
    }

    // calculate values for the tree bottom-up, which allows it to happen in a single pass.
    // Note: skipping over first line to makes sure that back-reference is valid.
    for &(ref current, ref next) in lines.iter().skip(1).rev() {
        let current = current.as_ref().expect("current");
        let next = next.borrow();

        for (l, v) in current.borrow_mut().iter_mut().enumerate() {
            let mut it = next.iter().skip(l);

            if let (Some(a), Some(b)) = (it.next(), it.next()) {
                *v = u64::max(a + *v, b + *v);
            } else {
                panic!("expected two values :(");
            }
        }
    }

    // first line contains maximum weight
    let first = lines[0].1.borrow();
    first[0]
}

problem!{
    tests => [
        q => {run(), "53f66b6783cb7552d83015df01b0d5229569fce1dd7d1856335c7244b9a3ded6"},
    ];
}
