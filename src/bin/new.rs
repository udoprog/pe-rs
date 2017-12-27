use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {
    let mut args = env::args();
    args.next();

    let problem: u64 = args.next().expect("number of problem").parse().expect("a problem number");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let path = root.join("src").join(format!("p{:03}.rs", problem));

    if path.is_file() {
        panic!("path already exists: {}", path.display());
    }

    println!("Writing: {}", path.display());

    let mut f = File::create(path).expect("bad file");

    f.write_all(b"/// Keywords: none

fn run() -> u64 {
    0
}

problem!{
    tests => [
        q => (run(), 0),
    ];
}
").expect("failed to write file");
}
