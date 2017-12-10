use std::io::Cursor;

use p067::run;

const INPUT: &str = include_str!("p018_triangle.txt");

problem!{
    tests => [
        q => {run(Cursor::new(INPUT)), "fde3f2e7127f6810eb4160bf7bb0563240d78c9d75a9a590b6d6244748a7f4ff"},
    ];
}
