use p016::run;

problem!{
    tests => [
        example1 => (run(2..=10), 27),
        q => {run(2..=100), "c86a2932e1c79343a3c16fb218b9944791aaeedd3e30c87d1c7f505c0e588f7c"},
    ];
}
