extern crate projecteuler;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let spoil = args.next().map(|a| a == "spoil").unwrap_or(false);

    projecteuler::run_all(spoil);
}
