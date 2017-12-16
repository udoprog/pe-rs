/// Keywords: pythagoras

fn run() -> u32 {
    for a in 1u32..=998 {
        for b in 1u32..=a {
            let c = 1000 - a - b;

            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c;
            }
        }
    }

    0
}

problem!{
    tests => [
        q => {run(), "d912d9d473ef86f12da1fb2011c5c0c155bd3a0ebdb4bbd7ea275cecdcb63731"},
    ];
}
