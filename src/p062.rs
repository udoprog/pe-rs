/// Keywords: none

use std::collections::HashMap;
use std::rc::Rc;

fn run() -> u64 {
    let mut first: HashMap<Rc<Vec<u8>>, u64> = HashMap::new();
    let mut lookup: HashMap<Rc<Vec<u8>>, u64> = HashMap::new();

    for num in 2u64.. {
        let cube = num * num * num;

        let mut digs = cube.to_string().chars().map(|c| c as u8).collect::<Vec<_>>();
        digs.sort();
        let digs = Rc::new(digs);

        let first = first.entry(Rc::clone(&digs)).or_insert(cube);
        let n = lookup.entry(Rc::clone(&digs)).or_insert_with(Default::default);

        *n += 1;

        if *n == 5 {
            return *first;
        }
    }

    panic!("not found");
}

problem!{
    tests => [
        q => {run(), "d25a595036aa8722157aca38f90084acb369b00df1070f49e203d5a3b7a0736d"},
    ];
}
