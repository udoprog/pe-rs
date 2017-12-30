/// Keywords: none

use digits::digits;

fn run() -> u64 {
    for n in 1.. {
        let mut a: Vec<u8> = digits(n).collect();
        let mut b: Vec<u8> = digits(n * 2).collect();
        let mut c: Vec<u8> = digits(n * 3).collect();
        let mut d: Vec<u8> = digits(n * 4).collect();
        let mut e: Vec<u8> = digits(n * 5).collect();
        let mut f: Vec<u8> = digits(n * 6).collect();

        a.sort();
        b.sort();
        c.sort();
        d.sort();
        e.sort();
        f.sort();

        if a == b && a == c && a == d && a == e && a == f {
            return n;
        }
    }

    panic!("no solution found");
}

problem!{
    tests => [
        q => {run(), "ebd72b510911af3e254a030cd891cb804e1902189eee7a0f6199472eb5e4dba2"},
    ];
}
