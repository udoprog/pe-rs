/// Keywords: pentagonal

fn run() -> u64 {
    let mut tri = (2..).map(triangle);
    let mut pen = (2..).map(pentagonal);
    let mut hex = (2..).map(hexagonal);

    let mut a = tri.next().expect("no tri");
    let mut b = pen.next().expect("no pen");
    let mut c = hex.next().expect("no hex");

    loop {
        if a != 40755 && a == b && b == c {
            return a;
        }

        c = hex.next().expect("no hex");

        while a < c {
            a = tri.next().expect("no tri");
        }

        while b < c {
            b = pen.next().expect("no tri");
        }
    }

    fn triangle(n: u64) -> u64 {
        (n * (n + 1)) / 2
    }

    fn pentagonal(n: u64) -> u64 {
        (n * (3 * n - 1)) / 2
    }

    fn hexagonal(n: u64) -> u64 {
        n * (2 * n - 1)
    }
}

problem!{
    tests => [
        q => {run(), "8b0300d71656b9cf0716318be9453c99a13bb8644d227fd683d06124e6a28b35"},
    ];
}
